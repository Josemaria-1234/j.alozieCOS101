fn main() {
    let mut m_heights = ("Everest", 8848, "Fishtail", 6693);
    println!("Original tuple = {:?}", m_heights);

    m_heights.2 = "Lhotse";
    m_heights.3 = 8516;

    println!("Changed tuple = {:?}", m_heights);
}