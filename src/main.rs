use clipboard::{ClipboardProvider, ClipboardContext};

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let qlip = ctx.get_contents().unwrap();
    qr2term::print_qr(qlip)
        .expect("Clipboard contents could not generate usable code.");
}
