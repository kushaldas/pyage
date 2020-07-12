import pyage

def test_createkeypair():
    public, secret = pyage.create_newkey()
    assert len(public) > 0
    assert secret.startswith("AGE-SECRET-KEY-")
