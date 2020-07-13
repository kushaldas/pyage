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


def test_encrypt_decrypt_armored_bytes():
    with open("tests/files/public.txt") as f:
        public = f.read()
    print(public)
    with open("tests/files/secret.txt") as f:
        secret = f.read()

    text = "Kushal loves 🐍 and 🦀"
    text_in_bytes = text.encode("utf-8")
    encrypted = pyage.encrypt_bytes(text_in_bytes, [public], armor=True)

    assert text_in_bytes != encrypted
    assert encrypted.startswith(b"-----BEGIN AGE ENCRYPTED FILE-----\n")

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

def test_encrypt_decrypt_files_armored():
    with open("tests/files/public.txt") as f:
        public = f.read()
    print(public)
    with open("tests/files/secret.txt") as f:
        secret = f.read()

    assert pyage.encrypt_file("/etc/hosts", "/tmp/ehost.age", [public,], armor=True)
    with open("/tmp/ehost.age") as f:
        line = f.readline()
        assert line == "-----BEGIN AGE ENCRYPTED FILE-----\n"
    assert pyage.decrypt_file("/tmp/ehost.age", "/tmp/hosts.txt", secret)


def test_encrypt_decrypt_bytes_withpassword():
    password = "ColaJailhouseLanternPopularCoagulantProofreadSedanActivate"
    text = "Kushal loves 🐍 and 🦀"
    text_in_bytes = text.encode("utf-8")
    encrypted = pyage.encrypt_bytes_withpassword(text_in_bytes, password)

    assert text_in_bytes != encrypted

    decrypted_in_bytes = pyage.decrypt_bytes_withpassword(encrypted, password)

    assert text == decrypted_in_bytes.decode("utf-8")
