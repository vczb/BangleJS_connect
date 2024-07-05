# Bangle JS 2 Connect App

This project is a Rust-based application designed to interface with the Bangle JS 2 smartwatch.

## Features

- Mouse move
- Keypress
- Click
- Printscren
- Volume Control
- Etc

**Status:** Experimental

## Getting Started

### Prerequisites

- Rust: [Install Rust](https://www.rust-lang.org/tools/install)
- Bluetooth-enabled device
- Bangle JS 2 smartwatch

### Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/vczb/BangleJS_connect.git
   cd BangleJS_connect
   cargo build --release
   cargo run --release
   ```

2. Build the project:

   ```sh
   cargo build --release
   ```

3. Run the application:

   ```sh
   cargo run --release
   ```

### Pairing with the Smartwatch via Bluetooth

1. Download the Bangle JS 2 Companion App:

   - Visit the Bangle JS 2 app loader [here](https://vczb.github.io/BangleApps/?q=pcconn).
   - Download and install the PC Connect (pcconn) app on your Bangle JS 2 smartwatch.

2. Restart the Smartwatch

3. Open the PC Conn app in the App menu

---

For any questions or issues, please open an issue on the GitHub repository. Enjoy using your Bangle JS 2 with this Rust application!
