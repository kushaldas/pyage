import pyage

def test_encrypt_decrypt_bytes():
    with open("tests/files/public.txt") as f:
        public = f.read()
    print(public)
    with open("tests/files/secret.txt") as f:
        secret = f.read()

    text = "Kushal loves 🐍 and 🦀"
    text_in_bytes = text.encode("utf-8")
    encrypted = pyage.encrypt_bytes(text_in_bytes, [public])

    assert text_in_bytes != encrypted

    decrypted_in_bytes = pyage.decrypt_bytes(encrypted, secret)

    assert text == decrypted_in_bytes.decode("utf-8")

def test_encrypt_decrypt_files():
    with open("tests/files/public.txt") as f:
        public = f.read()
    print(public)
    with open("tests/files/secret.txt") as f:
        secret = f.read()

    assert pyage.encrypt_file("/etc/hosts", "/tmp/ehost.age", [public,])
    assert pyage.decrypt_file("/tmp/ehost.age", "/tmp/hosts.txt", secret)

