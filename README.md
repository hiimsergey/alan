# Alan
A simple Enigma-like string encryption/decryption algorithm. Made with a bit over 30 lines of code.

## Example
```
$ alan mySecureKey "Another one bites the dust"
½ä×Í¸ÈìÑ´{±Þ×ÊÆv×åÈo¿Äè×

$ alan -d mySecureKey ½ä×Í¸ÈìÑ´{±Þ×ÊÆv×åÈo¿Äè×
Another one bites the dust
```

But you can also `cat` files:

```
$ alan mySecureKey "Another one bites the dust" > secret.txt

$ alan -d mySecureKey $(cat secret.txt)
Another one bites the dust
```
