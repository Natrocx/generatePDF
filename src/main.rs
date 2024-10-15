use generatePDF::generate_pdf_with_size;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let file_name = &args[1];
    let file_size_bytes: usize = args[2].parse().unwrap();
    generate_pdf_with_size(file_size_bytes).unwrap()
        .save(file_name)
        .unwrap();
}
