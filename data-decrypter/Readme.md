Try Hack Me - Room - Rust - Task 13 - Challenge


Challenge file 1.

"M3I6r2IbMzq9" is the text.

The text is encrypted with:

plaintext -> ROT13 -> base64 -> rot13

Go the opposite way and decrypt the file.

rot13 -> base64 -> ROT13 -> plaintext

You'll notice it's the same order either way, so you don't have to worry about order. Just make sure ROT13 is on the inside.

You might run into lifetime borrow checker issues.

Here's some hints in case you do:

1. Google is your friend.

2. https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

3. Stop trying to do so many things at once. Break it down to the bare necessities and slowly build back up to see what causes the errors.
