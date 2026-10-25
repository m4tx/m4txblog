---
title: "Framework 13 Pro from a Linux user perspective"
permalink: "framework-13-pro-linux-perspective"
date: 2030-01-01 12:00:00+0000
---

## My use case

TODO

Mostly using a desktop PC, laptop for travel, occasional home use, not for commercial work

## Is this a "Linux MacBook"?

This really depends on what that means to you exactly, but the answer is somewhere between "it's close" to "it's much better". For a price comparable to a MacBook Pro you get:

* Amazing repairability and upgradability - MacBook's soldered everything (including RAM and even storage nowadays!) is not even in the race.
* A very solid build quality, a comfortable keyboard and a great touchpad
* Customization options that no other vendor matches.

# Tweaks I've done

The kernel command line is in `/etc/default/limine`, one `KERNEL_CMDLINE[default]+="..."` line per group.
After editing it, run `sudo limine-update` and reboot.

## Display

### Disable HDR and wide colour gamut
With HDR on, the backlight stays at 100% and brightness is reduced in software. The panel isn't really an HDR panel anyway.

- System Settings → Display & Monitor → eDP-1: turn off **HDR** and **Wide Color Gamut**.

### Set VRR to Automatic
The xe driver doesn't enable PSR (Panel Self Refresh) while VRR is active. With *Automatic*, VRR is only active for fullscreen games.

- System Settings → Display & Monitor → eDP-1 → Adaptive sync: **Automatic**.

Result of the display changes: about 0.7W on idle.

You can verify that PSR is working with the following command:

```bash
sudo cat /sys/kernel/debug/dri/0/i915_edp_psr_status
```

It should give an output like this:

```
Sink support: PSR = yes [0x03], Panel Replay = no, Panel Replay Selective Update = no, Panel Replay DSC support = not supported
PSR mode: PSR2 enabled
Source PSR/PanelReplay ctl: enabled [0x8000c399]
Source PSR/PanelReplay status: SU_STANDBY [0x68200011]
Busy frontbuffer bits: 0x00000000
Performance counter: 0
PSR2 selective fetch: enabled
```

## Expansion cards

### Replace the HDMI card with a USB-C card
The HDMI card has an active converter inside, and it kept the Type-C subsystem powered even with nothing connected. Swapping it saved about 0.5 W.

- Only plug in the HDMI card when you need it.

## Power management (tuned)

### Custom tuned profile for power-saver mode
It builds on `cachyos-powersave` and adds two settings that powertop flagged. tuned applies them only in power-saver mode and undoes them when the profile changes.
- **Xe GPU `power_saving` profile**, which allows lower GPU clocks.
- **Runtime power management for all PCI devices.** Wi-Fi, Bluetooth, NVMe, CSME and others were pinned `on`.

- Create `/etc/tuned/profiles/custom-powersave/tuned.conf`:
  ```ini
  [main]
  include=cachyos-powersave

  [sysfs]
  /sys/class/drm/card*/device/tile*/gt*/freq0/power_profile=power_saving
  /sys/bus/pci/devices/*/power/control=auto
  ```
- In `/etc/tuned/ppd.conf`, set `power-saver=custom-powersave`.
- Run:
  ```bash
  sudo systemctl restart tuned tuned-ppd
  ```

### YubiKey autosuspend

I have a YubiKey permanently connected to a USB-A port. By default, it never suspends, which keeps the USB controller powered. 

- Create `/etc/udev/rules.d/50-yubikey-autosuspend.rules`:
  ```
  ACTION=="add", SUBSYSTEM=="usb", ATTR{idVendor}=="1050", TEST=="power/control", ATTR{power/control}="auto"
  ```
- `sudo udevadm control --reload`, then replug the key.
- Check: `power/runtime_status` of the device should become `suspended`.

### Lazy RCU
This batches the kernel's RCU callbacks for up to about 10 s instead of processing them right away, so CPUs wake less often. The gain is small, and the CachyOS kernel ships it disabled by default.

- Add `rcutree.enable_rcu_lazy=1 rcu_nocbs=all` to `/etc/default/limine`.
- Check: `cat /sys/module/rcutree/parameters/enable_rcu_lazy` should print `Y`.

## Memory

### Replace zram with zswap and a disk swap file
zram has no disk behind it, so the out-of-memory killer starts closing apps once it's full. zswap keeps recently swapped pages compressed in RAM and moves cold ones to an encrypted swap file on disk, so you can use more memory. Neither affects battery noticeably.

- Add `systemd.zram=0 zswap.enabled=1 zswap.max_pool_percent=30` to `/etc/default/limine`.
- Create an empty `/etc/udev/rules.d/30-zram.rules`. It overrides CachyOS's zram rule, which can turn zswap off.
- Create the swap file:
  ```bash
  sudo btrfs subvolume create /swap
  sudo btrfs filesystem mkswapfile --size 32g /swap/swapfile
  echo '/swap/swapfile none swap defaults 0 0' | sudo tee -a /etc/fstab
  ```
- As CachyOS's default swappiness of 150 is tuned for zram where swapping is essentially free, reduce the swappiness a little bit:
  ```bash
  echo 'vm.swappiness = 100' | sudo tee /etc/sysctl.d/99-swappiness.conf
  ```
- Check: `swapon --show` lists only the swap file, and `cat /sys/module/zswap/parameters/enabled` prints `Y`.

## Fingerprint authentication

### sudo
- In `/etc/pam.d/sudo`, add this as the first `auth` line:
  `auth sufficient pam_fprintd.so max-tries=1 timeout=10`

### polkit (KDE auth dialogs, run0)
The vendor `polkit-1` PAM file only asks for a password. A file with the same name in `/etc/pam.d/` takes precedence over it.

- Create `/etc/pam.d/polkit-1`:
  ```
  auth       sufficient   pam_fprintd.so max-tries=1 timeout=10
  auth       include      system-auth
  account    include      system-auth
  password   include      system-auth
  session    include      system-auth
  ```

### Lock screen
This already works out of the box through the `kde-fingerprint` PAM service. The "(or scan your fingerprint on the reader)" hint is missing because of a bug in kscreenlocker 6.8 (a regression from commit a5ed9ca0), not because of the configuration.

## Secure Boot

kernel lockdown

TODO

## Various (laptop-related?) settings

Short lock screen time, don't restore session, 
