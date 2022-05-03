# columba

Send encrypted messages over TCP.

The name 'Columba' derives from the genus of birds [Columba](https://en.wikipedia.org/wiki/Columba_(bird)), in which the original carrier pigeon, the rock dove, resides.

This project is ment to be a learning experience rather than a real product. While it may prove to be secure enough to use, I do not guarantee this is the case or always will be the case, so use at your own risk.

Example use of cli:

```
$ columba-cli connect <URL>
```

## notice

crypto is handled by the implementors of the client, e.g. `columba-cli`, the client by default does not encrypt messages

the package columba-crypto is ment to help with this implementation

this is important to note if you wish to make your own implementation
