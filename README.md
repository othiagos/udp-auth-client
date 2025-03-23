# Basic UDP Authentication Token Generator

## Introduction
In this assignment, we will develop an authenticator of student groups. The authentication protocol is capable of authenticating students individually or in groups. We will use the authenticator in the follow-up assignments.

## Objectives
- Introduce the socket programming interface
- Introduce the concepts of client and server applications
- Introduce the concepts of data encoding and transmission

## Implementation
This programming assignment can be implemented in any programming language, but using the low-level POSIX socket interface. Although the POSIX interface is the simplest available, it is also the "common denominator" when it comes to network applications. By using the POSIX interface, we will get a glimpse of the foundations under more advanced libraries and frameworks.

## Protocol
The authentication protocol used in this assignment is a request-response protocol. The client sends a message to the server and waits for a response. Authentications are done in at least two steps:
1. The client requests the server an authentication token.
2. After receiving the authentication token, the client can authenticate itself infinite times using the token, for example, to access other functionalities in the application.

The communication protocol uses UDP messages. Error detection and message retransmission in case of failure is the responsibility of the client.

The two common transport protocols on the Internet are TCP (Transmission Control Protocol) and UDP (User Datagram Protocol). We will study them in detail in this course, but for now, it suffices to know that TCP implements error and congestion detection, while UDP does not. If the library you are using has a default transport protocol, it is likely TCP; in this case, set the transport protocol to UDP manually.

The protocol transmits data in binary form. All integers transferred by the protocol should be encoded in network byte order. Each message starts with a 2-byte integer indicating the message type, and the semantics of the following bytes are defined for each message type.

## Message Types
### Individual Token Request [1]
```
0         2                        14                  18
+----+----+----+----/    /----+----+----+----+----+----+
| 1       | ID                     | nonce             |
+----+----+----+----/    /----+----+----+----+----+----+
```

### Individual Token Response [2]
```
0       2               14              18                    82
+---+---+---+---/   /---+---+---+---+---+---+---/         /---+
| 2     | ID            | nonce         | token               |
+---+---+---+---/   /---+---+---+---+---+---+---/         /---+
```

### Individual Token Validation [3]
```
0       2               14              18                    82
+---+---+---+---/   /---+---+---+---+---+---+---/         /---+
| 3     | ID            | nonce         | token               |
+---+---+---+---/   /---+---+---+---+---+---+---/         /---+
```

### Individual Token Status [4]
```
0       2               14              18                    82  83
+---+---+---+---/   /---+---+---+---+---+---+---/         /---+---+
| 4     | ID            | nonce         | token               | s |
+---+---+---+---/   /---+---+---+---+---+---+---/         /---+---+
```

### Group Token Request [5]
```
0       2       4          84         164       4+80N
+---+---+---+---+--/     /--+--/     /--+--/     /--+
| 5     | N     | SAS-1     | SAS-2     | SAS-N     |
+---+---+---+---+--/     /--+--/     /--+--/     /--+
```

### Group Token Response [6]
```
0       2       4          84         164       4+80N         4+80N+64
+---+---+---+---+--/    /--+--/     /--+--/     /--+--/   /--+
| 6     | N     | SAS-1    | SAS-2     | SAS-N     | token   |
+---+---+---+---+--/    /--+--/     /--+--/     /--+--/   /---
```

### Error Message [256]
```
0         2         4
+----+----+----+----+
| 256     | error   |
+----+----+----+----+
```

## Error Codes
- `INVALID_MESSAGE_CODE = 1` - Unknown request type.
- `INCORRECT_MESSAGE_LENGTH = 2` - Incompatible message size.
- `INVALID_PARAMETER = 3` - Invalid field value.
- `INVALID_SINGLE_TOKEN = 4` - Invalid SAS in a GAS.
- `ASCII_DECODE_ERROR = 5` - Non-ASCII character detected.

### Command-Line Interface
```
./client <host> <port> <command>
```

#### Commands
- `itr <id> <nonce>` - Request individual token.
- `itv <SAS>` - Validate individual token.
- `gtr <N> <SAS-1> <SAS-2> ... <SAS-N>` - Request group token.
- `gtv <GAS>` - Validate group token.

### Example Usage
```
% ./client vcm-23691.vm.duke.edu 51001 itr ifs4 1
ifs4:1:2c3bb3f0e946a1afde7d9d0c8c818762a6189e842abd8aaaf85c9faac5b784d2

% ./client vcm-23691.vm.duke.edu 51001 itv ifs4:1:2c3bb3f0e946a1afde7d9d0c8c818762a6189e842abd8aaaf85c9faac5b784d2
0

% ./client vcm-23691.vm.duke.edu 51001 itv ifs4:5:2c3bb3f0e946a1afde7d9d0c8c818762a6189e842abd8aaaf85c9faac5b784d2
1
```

## How to Compile and Run

To compile and run the Rust program using Cargo, follow the instructions below:

### Compilation
Run the following command in the terminal inside the project directory:
```sh
cargo build --release
```
This will generate the optimized binary in the `target/release/` folder.

### Execution
To run the program without manually compiling, use:
```sh
cargo run
```
Or, to execute the optimized version after compilation:
```sh
target/release/udp-auth-client
```