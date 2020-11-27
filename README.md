# Adobe License Decoder

[![Rust CI (stable)](https://github.com/adobe/adobe-license-decoder.rs/workflows/Rust%20CI%20(stable)/badge.svg)](https://github.com/adobe/adobe-license-decoder.rs/actions?query=workflow%3A%22Rust+CI+%28stable%29%22)

Anyone who has worked with FRL or SDL licensing is familiar with the adobe-licensing-toolkit command-line tool for Mac and Windows.  This tool runs on client machines in the context of a particular user account and provides information about the state of FRL and SDL licenses that are installed on the machine, including: 

- the so-called "npdId" (also known as the "package id") of the license;
- whether the license is activated for the given user; and
- if activated, what the expiration date is of the license.

While this information is invaluable, it's specific to the user account it is run in, and it doesn't give any general information about the licenses that are installed on the machine that haven't been used.

Enter the `adobe-license-decoder`, a different command-line tool that can tell you about FRL and SDL license files both before and after installation.  This tool can examine globally-installed SDL and FRL license files and tell you which apps they are for, which packages they are from, when they were installed, when they expire, and so on.  It's like a "secret decoder ring" for the licenses!

## Installation

The adobe-license-decoder is a command line tool that doesn't require any special privileges.  So to install it on a given machine, just download the Mac or Win binary from the [latest release page](https://github.com/adobeDan/adobe-license-decoder.rs/releases/latest).  Those can be used from any command line.

## Usage

If invoked without any command-line arguments, the adobe-license-decoder will look for a globally installed OperatingConfigs directory, and decode all the license files found in that directory.

If you have some other directory that you want it to look in for license files (for example, if your customer zipped up their OperatingConfigs directory and sent the zip to you), just name that directory on the command line, as in:

```
adobe-license-decoder customer-license-files
```

If you have a license-only package, you can invoke the decoder on the preconditioning installation file in that package (which is the `.json`) file. For example:

```
adobe-license-decoder my-package/ngl-preconditioning-data.json
```

## How to Read the Decoder's Reports

The following is a sample run of the adobe-license-decoder tool on a FRL Online package that has been set up to use a proxy server.  It shows the common data for the package at the top, followed by a list of the applications licensed by the package.  You can see immediately that it's an FRL Online package, that it was built with a custom server endpoint (the local proxy server address), that it's for a CC All Apps license, and so on.

```
$ adobe-license-decoder frl-online-with-proxy/ngl-preconditioning-data.json
Preconditioning data for npdId: ZTYyNjZiM2MtOTM1ZC00N2NiLTkzYmYtNGRkYWEzYTgxMWQ4
    Package UUID: e6266b3c-935d-47cb-93bf-4ddaa3a811d8
    License type: FRL Online/Connected (server: https://frl-proxy.brotsky.net:8443)
    License expiry date: controlled by server
    Precedence: 90 (CC All Apps)
Application Licenses (AppID, Certificate Group):
 1: AcrobatDC1, 2018072004
 2: AfterEffects1, 2018072004
 3: Animate1, 2018072004
 4: Audition1, 2018072004
 5: Bridge1, 2018072004
 6: CharacterAnimator1, 2018072004
 7: Dreamweaver1, 2018072004
 8: Illustrator1, 2018072004
 9: InCopy1, 2018072004
10: InDesign1, 2018072004
11: LightroomClassic1, 2018072004
12: MediaEncoder1, 2018072004
13: Photoshop1, 2018072004
14: Prelude1, 2018072004
15: PremierePro1, 2018072004
```

Suppose we were to install the package above, using this command line (on Mac):

```
sudo adobe-licensing-toolkit -p -i -f frl-online-with-proxy/ngl-preconditioning-data.json
```

Then we could run the decoder with no arguments, and it would find the installed operating config files (as shown in the run below).  Since all the license files are for the same package, it still groups the package-specific information at the top of the list (but notice it now says "License files for" instead of "Preconditioning data for").  Then it shows the license-file-specific info for each of the licenses that are installed, giving the filename of the relevant operating configuration file (elided so it doesn't repeat the npdId segment of the filename each time), the specific application that license file is for, and the install date of the license file.  The install date is important, because on a machine that has multiple packages installed, and thus has multiple license files of the same precedence for the same application, it's the most recently installed license file that will be used by the app when it launches.

```
$ adobe-license-decoder
License files for npdId: ZTYyNjZiM2MtOTM1ZC00N2NiLTkzYmYtNGRkYWEzYTgxMWQ4:
    Package UUID: e6266b3c-935d-47cb-93bf-4ddaa3a811d8
    License type: FRL Online/Connected (server: https://frl-proxy.brotsky.net:8443)
    License expiry date: controlled by server
    Precedence: 90 (CC All Apps)
Filenames (shown with '...' where the npdId appears):
 1: QWNyb2JhdERDMXt9MjAxODA3MjAwNA-...-90.operatingconfig
    App ID: AcrobatDC1, Certificate Group: 2018072004
    Install date: 2020-11-07 11:38:05 -08:00
 2: QWZ0ZXJFZmZlY3RzMXt9MjAxODA3MjAwNA-...-90.operatingconfig
    App ID: AfterEffects1, Certificate Group: 2018072004
    Install date: 2020-11-07 11:38:11 -08:00
 3: QW5pbWF0ZTF7fTIwMTgwNzIwMDQ-...-90.operatingconfig
    App ID: Animate1, Certificate Group: 2018072004
    Install date: 2020-11-07 11:38:04 -08:00
 4: QXVkaXRpb24xe30yMDE4MDcyMDA0-...-90.operatingconfig
    App ID: Audition1, Certificate Group: 2018072004
    Install date: 2020-11-07 11:38:01 -08:00
 5: QnJpZGdlMXt9MjAxODA3MjAwNA-...-90.operatingconfig
    App ID: Bridge1, Certificate Group: 2018072004
    Install date: 2020-11-07 11:38:06 -08:00
 6: Q2hhcmFjdGVyQW5pbWF0b3Ixe30yMDE4MDcyMDA0-...-90.operatingconfig
    App ID: CharacterAnimator1, Certificate Group: 2018072004
    Install date: 2020-11-07 11:38:13 -08:00
 7: RHJlYW13ZWF2ZXIxe30yMDE4MDcyMDA0-...-90.operatingconfig
    App ID: Dreamweaver1, Certificate Group: 2018072004
    Install date: 2020-11-07 11:38:10 -08:00
 8: SWxsdXN0cmF0b3Ixe30yMDE4MDcyMDA0-...-90.operatingconfig
    App ID: Illustrator1, Certificate Group: 2018072004
    Install date: 2020-11-07 11:38:00 -08:00
 9: SW5Db3B5MXt9MjAxODA3MjAwNA-...-90.operatingconfig
    App ID: InCopy1, Certificate Group: 2018072004
    Install date: 2020-11-07 11:38:08 -08:00
10: SW5EZXNpZ24xe30yMDE4MDcyMDA0-...-90.operatingconfig
    App ID: InDesign1, Certificate Group: 2018072004
    Install date: 2020-11-07 11:38:07 -08:00
11: TGlnaHRyb29tQ2xhc3NpYzF7fTIwMTgwNzIwMDQ-...-90.operatingconfig
    App ID: LightroomClassic1, Certificate Group: 2018072004
    Install date: 2020-11-07 11:38:12 -08:00
12: TWVkaWFFbmNvZGVyMXt9MjAxODA3MjAwNA-...-90.operatingconfig
    App ID: MediaEncoder1, Certificate Group: 2018072004
    Install date: 2020-11-07 11:38:09 -08:00
13: UGhvdG9zaG9wMXt9MjAxODA3MjAwNA-...-90.operatingconfig
    App ID: Photoshop1, Certificate Group: 2018072004
    Install date: 2020-11-07 11:38:02 -08:00
14: UHJlbHVkZTF7fTIwMTgwNzIwMDQ-...-90.operatingconfig
    App ID: Prelude1, Certificate Group: 2018072004
    Install date: 2020-11-07 11:38:14 -08:00
15: UHJlbWllcmVQcm8xe30yMDE4MDcyMDA0-...-90.operatingconfig
    App ID: PremierePro1, Certificate Group: 2018072004
    Install date: 2020-11-07 11:38:03 -08:00
```

Next, let's look at the information given about an FRL Isolated (or Offline) license.  Here we run the toolkit before installing the package, then install the package, then run the toolkit again afterwards.  Notice that, since this license doesn't contact a server, its expiration date is built into the package, so the decoder can tell you when the license will expire - this date includes the one-month grace past contract end we always give; it's the date that the apps will actually stop working.  Also, notice that this is a single-app license, as revealed by its precedence.  Finally, notice that the install dates don't match the order in which the files are listed: that's because the listings are always sorted by Application ID, but the adobe-licensing-toolkit installation is done in the order the app entries happen to appear in the preconditioning file. 

```
$ adobe-license-decoder isolated/ngl-preconditioning-data.json
Preconditioning data for npdId: MzEwYmEzMGYtMDJiNy00MzU0LThjZDUtM2RiNGJjOTU0YTFi
    Package UUID: 310ba30f-02b7-4354-8cd5-3db4bc954a1b
    License type: FRL Offline/Isolated
    License expiry date: 2021-11-04
    Precedence: 80 (CC Single App)
Application Licenses (AppID, Certificate Group):
 1: Bridge1, 2018072004
 2: InDesign1, 2018072004
 3: MediaEncoder1, 2018072004
$ sudo adobe-licensing-toolkit -p -i -f isolated/ngl-preconditioning-data.json
Adobe Licensing Toolkit (1.1.0.91)
Operation Successfully Completed
$ adobe-license-decoder
License files for npdId: MzEwYmEzMGYtMDJiNy00MzU0LThjZDUtM2RiNGJjOTU0YTFi:
    Package UUID: 310ba30f-02b7-4354-8cd5-3db4bc954a1b
    License type: FRL Offline/Isolated
    License expiry date: 2021-11-04
    Precedence: 80 (CC Single App)
Filenames (shown with '...' where the npdId appears):
 1: QnJpZGdlMXt9MjAxODA3MjAwNA-...-80.operatingconfig
    App ID: Bridge1, Certificate Group: 2018072004
    Install date: 2020-11-08 12:21:58 -08:00
 2: SW5EZXNpZ24xe30yMDE4MDcyMDA0-...-80.operatingconfig
    App ID: InDesign1, Certificate Group: 2018072004
    Install date: 2020-11-08 12:21:59 -08:00
 3: TWVkaWFFbmNvZGVyMXt9MjAxODA3MjAwNA-...-80.operatingconfig
    App ID: MediaEncoder1, Certificate Group: 2018072004
    Install date: 2020-11-08 12:21:57 -08:00
```

Finally, let's look at a run where we have installed a LAN package on top of the Isolated package.  (As above, we run the decoder over the package, then we install it, then we run the decoder to see what license files are on the machine.).  Because the LAN package and the Isolated package are both single-app packages, their licenses have the same precedence, so where there are two license files for the same application (in this case, Bridge), the LAN package will win because it has the later installation date.  It's in situations like these - where customers have installed two different packages on top of each other, that the decoder tool can really come in handy in understanding what's happened and in getting it fixed.

```
$ adobe-license-decoder lan/ngl-preconditioning-data.json
Preconditioning data for npdId: ZWQzOWI4NWQtYjg0Zi00NTExLWEwYmUtOTIwYzc1NmY5YTZk
    Package UUID: ed39b85d-b84f-4511-a0be-920c756f9a6d
    License type: FRL LAN (server: https://test:123)
    License expiry date: controlled by server
    Precedence: 80 (CC Single App)
Application Licenses (AppID, Certificate Group):
 1: Bridge1, 2018072004
 2: Illustrator1, 2018072004
$ sudo adobe-licensing-toolkit -p -i -f lan/ngl-preconditioning-data.json
Password:
Adobe Licensing Toolkit (1.1.0.91)
Operation Successfully Completed
$ adobe-license-decoder
License files for npdId: MzEwYmEzMGYtMDJiNy00MzU0LThjZDUtM2RiNGJjOTU0YTFi:
    Package UUID: 310ba30f-02b7-4354-8cd5-3db4bc954a1b
    License type: FRL Offline/Isolated
    License expiry date: 2021-11-04
    Precedence: 80 (CC Single App)
Filenames (shown with '...' where the npdId appears):
 1: QnJpZGdlMXt9MjAxODA3MjAwNA-...-80.operatingconfig
    App ID: Bridge1, Certificate Group: 2018072004
    Install date: 2020-11-08 12:21:58 -08:00
 2: SW5EZXNpZ24xe30yMDE4MDcyMDA0-...-80.operatingconfig
    App ID: InDesign1, Certificate Group: 2018072004
    Install date: 2020-11-08 12:21:59 -08:00
 3: TWVkaWFFbmNvZGVyMXt9MjAxODA3MjAwNA-...-80.operatingconfig
    App ID: MediaEncoder1, Certificate Group: 2018072004
    Install date: 2020-11-08 12:21:57 -08:00
License files for npdId: ZWQzOWI4NWQtYjg0Zi00NTExLWEwYmUtOTIwYzc1NmY5YTZk:
    Package UUID: ed39b85d-b84f-4511-a0be-920c756f9a6d
    License type: FRL LAN (server: https://test:123)
    License expiry date: controlled by server
    Precedence: 80 (CC Single App)
Filenames (shown with '...' where the npdId appears):
 4: QnJpZGdlMXt9MjAxODA3MjAwNA-...-80.operatingconfig
    App ID: Bridge1, Certificate Group: 2018072004
    Install date: 2020-11-08 12:33:48 -08:00
 5: SWxsdXN0cmF0b3Ixe30yMDE4MDcyMDA0-...-80.operatingconfig
    App ID: Illustrator1, Certificate Group: 2018072004
    Install date: 2020-11-08 12:33:47 -08:00
```

## Support

This tool is maintained by the Adobe DME Premium Onboarding team.  If you need support or just have questions about the `adobe-license-decoder`, please file an issue against this project.

## Contributing

Contributions are welcomed! Read the [Contributing Guide](./.github/CONTRIBUTING.md) for more information.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for more information.
