API Documentation
==================


create_newkey()
----------------


`pyage.create_newkey()` method returns a tuple of a new keypair as (publickey, secretkey).

::

        >>> public, secret = pyage.create_newkey()

.. note:: Remember to save both the public key and the secret key in proper places.

encrypt_bytes()
---------------

`pyage.encrypt_bytes()` takes three arguments, first argument is the data to be encrypted in bytes, and a list of
public keys, and an optional `armor`, and it returns encrypted text as bytes, or ascii-armored value if you pass `armor=True`.

::

        >>> data = b"The secret text"
        >>> key = "age1spp8yf63x4xu7l5esxlnzldwgmaaqmwrjw38vra9s7hw63pyzpqsq82gst"
        >>> encrypted_btyes = pyage.encrypt_btyes(data, [key,])


decrypt_bytes()
---------------- 

`pyage.decrypt_bytes()` takes two arguments, first argument is the encrypted data as bytes, and then the secret key as string.
It returns the decrypted bytes on success.

::

        >>> secret = "AGE-SECRET-KEY-19Z8Q85A9RTCLJ36EXCCCX0R6PTL0RPJ93JUZW48H7FLRJMSTV32S5XY2FA"
        >>> decrypted_bytes = pyage.decrypt_bytes(encrypt_btyes, secret)


encrypt_file()
-------------- 

`pyage.encrypt_file()` takes 3 arguments, first the input file path as str, the output file path (as str) is the second argument, and the
last argument is the list of public keys.

::

        >>> key = "age1spp8yf63x4xu7l5esxlnzldwgmaaqmwrjw38vra9s7hw63pyzpqsq82gst"
        >>> assert pyage.encrypt_file("/etc/hosts", "/tmp/hosts.age", [key,])

decrypt_file()
--------------

`pyage.decrypt_file()` takes 3 arguments, first the encrypted file path, then new decrypted output filepath, and then secret key. Returns `true`
in case of success.

::

        >>> secret = "AGE-SECRET-KEY-19Z8Q85A9RTCLJ36EXCCCX0R6PTL0RPJ93JUZW48H7FLRJMSTV32S5XY2FA"
        >>>> assert pyage.decrypt_file("/tmp/host.age", "/tmp/hosts", secret)

