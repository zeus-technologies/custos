rule contains_rust {
  strings:
    $rust = "rust" nocase
  condition:
    $rust
}
