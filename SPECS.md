create:

create an alias/username for a public key

username should be max 32 bytes

req:

```
create
name
<name>
public
<public>
```

res:

```
public
<public>
```

inbox:

req:

```
inbox
name
<name>
```

res:
```
inbox
<id> <date>
(...)
```

read:

```
read
id
<post id>
name
<name>
```

send:

```
send
name
<name>
message
<message>
```
