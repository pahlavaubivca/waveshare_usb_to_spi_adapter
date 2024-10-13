# How to use

## Copy libch347.so to /usr/local/lib

Add to .profile or .bashrc
```bash
export LD_LIBRARY_PATH="/usr/local/lib"
```

Run `source .profile` or `source .bashrc`

Copy libch347.so to /usr/local/lib
```bash
sudo cp lib/libch347.so /usr/local/lib
```


## Run with sudo
`cd target/debug`

`sudo LD_LIBRARY_PATH=$LD_LIBRARY_PATH ./waveshare_usb_to_spi_adapter`

## Rootless mode

#### Create Group
`sudo groupadd ch34x`

#### Apply target user to the group

`sudo usermod -aG ch34x <USER_NAME>`

> note: after this logout and login again

#### Add udev rule

`sudo nano /etc/udev/rules.d/99-ch34x.rules`

```bash
ATTRS{idVendor}=="1a86", ATTRS{idProduct}=="55db", MODE="664", GROUP="ch34x"
```
> note: idVendor and idProduct can be found using `lsusb`

Reload udev rules

```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```

#### Verify

`ls -l /dev/ch34x_pis6`