# Notes for the Invary ASPS

We need root perms for the invary asps to get the necessary data, but we do not want to have to be root to run them. Enter setuid

### Do the following to make it so anyone can run the invary asp successfully

```
sudo chown root:root ./bin/r_invary_get_measurement_id
sudo chmod u+s ./bin/r_invary_get_measurement_id
```

### NOTE: This will break the build so that the invary asp cannot be rebuilt, so if you want it to be rebuilt run the following

```
sudo rm ./bin/r_invary_get_measurement_id
make
```

Then repeat the above setuid trick again
