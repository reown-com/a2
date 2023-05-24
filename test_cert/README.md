This contains a self signed cert for test purposes. The password is
"test".

These values are not encrypted

Key and Cert generation (unencrypted):

```
$ openssl req -newkey rsa:2048 -nodes \
    -keyout test.key -x509 -days 3650 -out test.crt
```
