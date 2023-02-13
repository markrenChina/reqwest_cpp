error_chain! {
    foreign_links {
        Reqwest(::Reqwest::Error);
    }
}