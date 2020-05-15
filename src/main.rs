use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyle,
};

use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use bitcoin::network::constants::Network;
use bitcoin::secp256k1::rand::thread_rng;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::address::Address;
use bitcoin::util::key;

fn get_address() -> Address {
    // Generate random key pair
    let s = Secp256k1::new();
    let public_key = key::PublicKey {
        compressed: true,
        key: s.generate_keypair(&mut thread_rng()).1,
    };

    // Return pay-to-pubkey-hash address
    let address = Address::p2pkh(&public_key, Network::Bitcoin);
    return address;
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(300, 300));

    // Generate address and draw on screen
    let address = get_address();
    Text::new(&address.to_string(), Point::new(5, 5))
        .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
        .draw(&mut display)?;

    // Create window
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    Window::new(&address.to_string(), &output_settings).show_static(&display);

    Ok(())
}
