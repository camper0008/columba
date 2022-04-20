create:

create an alias/username for a public key

username should be max 128 bytes

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
posts
<id> <date>
```

read:

```
read
id
<id>
name
<name>
```

send:

```
send
reciepient
<name>
message
<message>
```
