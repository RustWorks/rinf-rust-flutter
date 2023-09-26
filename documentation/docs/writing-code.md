# How to Write Code

## 🔄 Request from Dart, Response from Rust

Let's say that you want to make a new button that sends an array of numbers and a string from Dart to Rust to perform some calculation on it. You can follow these steps to understand how to send a request and wait for the response.

Let's start from our [default example](https://github.com/cunarist/rust-in-flutter/tree/main/example).

Create a new `.proto` file in `./messages` that represents the new Rust resource.

```proto
// messages/tutorial_resource.proto

syntax = "proto3";
package tutorial_resource;

message ReadRequest {
  repeated int32 input_numbers = 1;
  string input_string = 2;
}

message ReadResponse {
  repeated int32 output_numbers = 1;
  string output_string = 2;
}
```

Next, generate Dart and Rust message code from `.proto` files.

```bash
rifs message
```

Create a button widget in Dart that accepts the user input.

```dart
// lib/main.dart
...
child: Column(
  mainAxisAlignment: MainAxisAlignment.center,
  children: [
    ElevatedButton(
      onPressed: () async {},
      child: Text("Request to Rust"),
    ),
...
```

`onPressed` function should send a request to Rust. Let's create a `RustRequest` object.

```dart
// lib/main.dart
...
import 'package:rust_in_flutter/rust_in_flutter.dart';
import 'package:example_app/messages/tutorial_resource.pb.dart'
    as tutorialResource;
...
ElevatedButton(
  onPressed: () async {
    final requestMessage = tutorialResource.ReadRequest(
      inputNumbers: [3, 4, 5],
      inputString: 'Zero-cost abstraction',
    );
    final rustRequest = RustRequest(
      resource: tutorialResource.ID,
      operation: RustOperation.Read,
      message: requestMessage.writeToBuffer(),
    );
    final rustResponse = await requestToRust(rustRequest);
  },
  child: Text("Request to Rust"),
),
...
```

`requestToRust` function sends the request to Rust, returning a `RustResponse` object.

Now, write our new endpoint Rust function `sample_functions::handle_tutorial_resource`. This simple API endpoint will add one to each element in the array, capitalize all letters in the string, and return them.

```rust
// native/hub/src/sample_functions.rs
...
use crate::bridge::api::{RustOperation, RustRequest, RustResponse, RustSignal};
...
pub async fn handle_tutorial_resource(rust_request: RustRequest) -> RustResponse {
    use crate::messages::tutorial_resource::{ReadRequest, ReadResponse};

    match rust_request.operation {
        RustOperation::Create => RustResponse::default(),
        RustOperation::Read => {
            let message_bytes = rust_request.message.unwrap();
            let request_message = ReadRequest::decode(message_bytes.as_slice()).unwrap();

            let new_numbers: Vec<i32> = request_message
                .input_numbers
                .into_iter()
                .map(|x| x + 1)
                .collect();
            let new_string = request_message.input_string.to_uppercase();

            let response_message = ReadResponse {
                output_numbers: new_numbers,
                output_string: new_string,
            };
            RustResponse {
                successful: true,
                message: Some(response_message.encode_to_vec()),
                blob: None,
            }
        }
        RustOperation::Update => RustResponse::default(),
        RustOperation::Delete => RustResponse::default(),
    }
}
...
```

The name of the new Rust resource was `tutorial_resource`. Make sure that the request handler function in Rust accepts this.

```rust
// native/hub/src/with_request.rs
...
use crate::bridge::api::{RustRequestUnique, RustResponse, RustResponseUnique};
use crate::messages;
use crate::sample_functions;
...
let rust_resource = rust_request.resource;
let rust_response = match rust_resource {
    messages::counter_number::ID => sample_functions::handle_counter_number(rust_request).await,
    messages::sample_folder::sample_resource::ID => {
        sample_functions::handle_sample_resource(rust_request).await
    }
    messages::sample_folder::deeper_folder::deeper_resource::ID => {
        sample_functions::handle_sample_resource(rust_request).await
    }
    messages::tutorial_resource::ID => {
        sample_functions::handle_tutorial_resource(rust_request).await // ADD THIS BLOCK
    }
    _ => RustResponse::default(),
};
...
```

Finally, when you receive a response from Rust in Dart, you can do anything with the bytes data in it.

```dart
// lib/main.dart
...
import 'package:rust_in_flutter/rust_in_flutter.dart';
import 'package:example_app/messages/tutorial_resource.pb.dart'
    as tutorialResource;
...
    final rustResponse = await requestToRust(rustRequest);
    final responseMessage =
        tutorialResource.ReadResponse.fromBuffer(
      rustResponse.message!,
    );
    print(responseMessage.outputNumbers);
    print(responseMessage.outputString);
  },
  child: Text("Request to Rust"),
),
...
```

And we can see the printed output in the command-line!

```
flutter: [4, 5, 6]
flutter: ZERO-COST ABSTRACTION
```

We just simply print the message here, but the response data will be used for rebuilding Flutter widgets and updating states in real apps.

You can extend this RESTful API pattern and create hundreds and thousands of endpoints as you need. If you have a web background, this system might look familiar.

## 📡 Streaming from Rust to Dart

Let's say that you want to send increasing numbers every second from Rust to Dart. In this case, it would be inefficient for Dart to send requests repeatedly. This is where streaming is needed.

Let's start from our [default example](https://github.com/cunarist/rust-in-flutter/tree/main/example).

Define the Rust resource and message schema.

```proto
// messages/increasing_number.proto

syntax = "proto3";
package increasing_number;

message StateSignal { int32 current_number = 1; }
```

Generate Dart and Rust message code from `.proto` files.

```bash
rifs message
```

Define an async Rust function that runs forever, sending numbers to Dart every second.

```rust
// native/hub/src/sample_functions.rs
...
use crate::bridge::api::{RustOperation, RustRequest, RustResponse, RustSignal};
use crate::bridge::send_rust_signal;
...
pub async fn stream_increasing_number() {
    use crate::messages::increasing_number::{StateSignal, ID};

    let mut current_number: i32 = 1;
    loop {
        crate::sleep(std::time::Duration::from_secs(1)).await;

        let signal_message = StateSignal { current_number };
        let rust_signal = RustSignal {
            resource: ID,
            message: Some(signal_message.encode_to_vec()),
            blob: None,
        };
        send_rust_signal(rust_signal);

        current_number += 1;
    }
}
...
```

Spawn the async function in Rust.

```rust
// native/hub/src/lib.rs
...
mod sample_functions;
...
crate::spawn(sample_functions::stream_mandelbrot());
crate::spawn(sample_functions::stream_increasing_number()); // ADD THIS LINE
while let Some(request_unique) = request_receiver.recv().await {
...
```

Finally, receive the signals in Dart with `StreamBuilder`, filter them by resource with the `where` method, and rebuild the widget.

```dart
// lib/main.dart
...
import 'package:rust_in_flutter/rust_in_flutter.dart';
import 'package:example_app/messages/increasing_number.pb.dart'
    as increasingNumbers;
...
children: [
  StreamBuilder<RustSignal>(
    stream: rustBroadcaster.stream.where((rustSignal) {
      return rustSignal.resource == increasingNumbers.ID;
    }),
    builder: (context, snapshot) {
      final rustSignal = snapshot.data;
      if (rustSignal == null) {
        return Text("Nothing received yet");
      } else {
        final singal = increasingNumbers.StateSignal.fromBuffer(
          rustSignal.message!,
        );
        final currentNumber = singal.currentNumber;
        return Text(currentNumber.toString());
      }
    },
  ),
...
```

We rebuild the widget with the received data here, but the streamed data can also be used to update Dart states in real apps.

## 🏷️ Meanings of Each Field

We've seen how to pass `RustRequest`, `RustResponse`, and `RustSignal` between Dart and Rust in this tutorial. Now let's go over to what exactly each field means.

### Field `resource`

This is an integer pointing to a virtual Rust resource that suits your app's design. Always provide `ID` of some message module generated by `rifs message`.

### Field `operation`

This accepts an enum value of `RustOperation` and can be one of create, read, update, and delete, since this system follows the definition of RESTful API.

### Field `message`

This is a bytes array created by Protobuf serialization. Note that it is not recommended to create Protobuf messages that are bigger than a few megabytes. To send large data, use `blob` instead. Sending bytes array is a zero-copy operation, though Protobuf serialization and deserialization process does involve memory copy. This field is optional and can be `null` or `None`.

### Field `blob`

This is also a bytes array intended to contain large data up to a few gigabytes. You can send any kind of binary as you wish such as a high-resolution image or some kind of file data. Sending a blob is a zero-copy operation, which means no memory copy is involved. This field is optional and can be `null` or `None`.