:: Author: ModelZ
:: Date: 13/04/2025
:: Description: Builds the Wicked Waifus servers in Windows Terminal.
:: Usage: Double-click this batch file to run it.
:: Requires: Windows Terminal installed and configured.

@echo off
cargo clean
cargo build --release