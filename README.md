Minigrep with rust from [rustbook](https://doc.rust-lang.org/stable/book/ch12-00-an-io-project.html)

#### Arguments

- <code>cargo run -- 'word to search' 'path to file'</code>
  </br> e.g. <code>cargo run -- to poem.txt</code>

#### Environment Variables

- IGNORE_CASE: If set to true, the search will be case insensitive
  <code>IGNORE_CASE=1 cargo run -- 'word to search' 'path to file'</code>
  </br> In powershell: <code>$env:IGNORE_CASE=1; cargo run -- 'word to search' 'path to file'</code>. Thiw will make persistent the environment variable for the current session. It can be removed with <code>Remove-Item Env:IGNORE_CASE</code>

- HARD_CASE: If set to true, the search will be case sensitive and searches for the exact word
  <code>HARD_CASE=1 cargo run -- 'word to search' 'path to file'</code>
  </br> In powershell: <code>$env:HARD_CASE=1; cargo run -- 'word to search' 'path to file'</code>. Thiw will make persistent the environment variable for the current session. It can be removed with <code>Remove-Item Env:HARD_CASE</code>
