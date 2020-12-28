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

If you have a license-only package, you can invoke the decoder on the directory contaning that package as well. For example:

```
adobe-license-decoder my-package-dir
```

Finally, if you have a single license file (a file whose name ends in `operatingconfig`) or a single preconditioning file (a `JSON` file that can be installed by the `adobe-licensing-toolkit`) that you want decoded, you can specify the name of the file itself instead of a directory, as in:

```
adobe-license-decoder ngl-preconditioning-data.json
```

In addition to the (optional) directory or file argument, the decoder takes an optional `-v` flag that causes the report it produces to be more verbose.  The next section details what this flag does.

## How to Read the Decoder's Reports

The following is a sample run of the adobe-license-decoder tool on a FRL Online package.  It shows the common data for the package at the top, followed by a list of the applications licensed by the package.  You can see immediately that it's an FRL Online package, that it was built against the standard server endpoint, that it's for a CC All Apps license, and so on.

```
$ adobe-license-decoder online-default-allapps
Preconditioning data for npdId: NzBjZmVlYWItNzc2Ni00ZTNiLTk4NjQtNjczYjc5ZDM2ZGRk
    License type: FRL Online (server: https://lcs-cops.adobe.io/)
    License expiry date: controlled by server
    Precedence: 90 (CC All Apps)
Application Licenses:
 1: App ID: AcrobatDC1
 2: App ID: AfterEffects1
 3: App ID: Animate1
 4: App ID: Audition1
 5: App ID: Bridge1
 6: App ID: CharacterAnimator1
 7: App ID: Dreamweaver1
 8: App ID: Illustrator1
 9: App ID: InCopy1
10: App ID: InDesign1
11: App ID: LightroomClassic1
12: App ID: MediaEncoder1
13: App ID: Photoshop1
14: App ID: Prelude1
15: App ID: PremierePro1
```

Suppose we were to install the package above, using this command line (on Mac):

```
$ sudo adobe-licensing-toolkit -p -i -f online-default-allapps/ngl-preconditioning-data.json
```

Then we could run the decoder with no arguments, and it would find the installed operating config files (as shown in the run below).  Since all the license files are for the same package, it still groups the package-specific information at the top of the list (but notice it now says "License files for" instead of "Preconditioning data for").  Then it shows the license-file-specific info for each of the licenses that are installed, giving the filename of the relevant operating configuration file (elided so it doesn't repeat the npdId segment of the filename each time), the specific application that license file is for, and the install date of the license file.  The install date is important, because on a machine that has multiple packages installed, and thus has multiple license files of the same precedence for the same application, it's the most recently installed license file that will be used by the app when it launches. (You may notice that the install dates don't match the order in which the files are listed: that's because the listings are always sorted by Application ID, but the adobe-licensing-toolkit installation is done in the order the app entries happen to appear in the preconditioning file.)

```
$ adobe-license-decoder
License files for npdId: NzBjZmVlYWItNzc2Ni00ZTNiLTk4NjQtNjczYjc5ZDM2ZGRk:
    License type: FRL Online (server: https://lcs-cops.adobe.io/)
    License expiry date: controlled by server
    Precedence: 90 (CC All Apps)
Filenames (shown with '...' where the npdId appears):
 1: QWNyb2JhdERDMXt9MjAxODA3MjAwNA-...-90.operatingconfig
    App ID: AcrobatDC1
    Install date: 2020-12-27 16:26:19 -08:00
 2: QWZ0ZXJFZmZlY3RzMXt9MjAxODA3MjAwNA-...-90.operatingconfig
    App ID: AfterEffects1
    Install date: 2020-12-27 16:26:18 -08:00
 3: QW5pbWF0ZTF7fTIwMTgwNzIwMDQ-...-90.operatingconfig
    App ID: Animate1
    Install date: 2020-12-27 16:26:06 -08:00
 4: QXVkaXRpb24xe30yMDE4MDcyMDA0-...-90.operatingconfig
    App ID: Audition1
    Install date: 2020-12-27 16:26:11 -08:00
 5: QnJpZGdlMXt9MjAxODA3MjAwNA-...-90.operatingconfig
    App ID: Bridge1
    Install date: 2020-12-27 16:26:17 -08:00
 6: Q2hhcmFjdGVyQW5pbWF0b3Ixe30yMDE4MDcyMDA0-...-90.operatingconfig
    App ID: CharacterAnimator1
    Install date: 2020-12-27 16:26:12 -08:00
 7: RHJlYW13ZWF2ZXIxe30yMDE4MDcyMDA0-...-90.operatingconfig
    App ID: Dreamweaver1
    Install date: 2020-12-27 16:26:09 -08:00
 8: SWxsdXN0cmF0b3Ixe30yMDE4MDcyMDA0-...-90.operatingconfig
    App ID: Illustrator1
    Install date: 2020-12-27 16:26:05 -08:00
 9: SW5Db3B5MXt9MjAxODA3MjAwNA-...-90.operatingconfig
    App ID: InCopy1
    Install date: 2020-12-27 16:26:07 -08:00
10: SW5EZXNpZ24xe30yMDE4MDcyMDA0-...-90.operatingconfig
    App ID: InDesign1
    Install date: 2020-12-27 16:26:16 -08:00
11: TGlnaHRyb29tQ2xhc3NpYzF7fTIwMTgwNzIwMDQ-...-90.operatingconfig
    App ID: LightroomClassic1
    Install date: 2020-12-27 16:26:13 -08:00
12: TWVkaWFFbmNvZGVyMXt9MjAxODA3MjAwNA-...-90.operatingconfig
    App ID: MediaEncoder1
    Install date: 2020-12-27 16:26:08 -08:00
13: UGhvdG9zaG9wMXt9MjAxODA3MjAwNA-...-90.operatingconfig
    App ID: Photoshop1
    Install date: 2020-12-27 16:26:10 -08:00
14: UHJlbHVkZTF7fTIwMTgwNzIwMDQ-...-90.operatingconfig
    App ID: Prelude1
    Install date: 2020-12-27 16:26:15 -08:00
15: UHJlbWllcmVQcm8xe30yMDE4MDcyMDA0-...-90.operatingconfig
    App ID: PremierePro1
    Install date: 2020-12-27 16:26:14 -08:00
```

Next, let's look at the information given about an FRL Isolated license.  Here we run the toolkit before installing the package, then install the package, then run the toolkit again afterwards.  Notice that, since this license doesn't contact a server, its expiration date is built into the package, so the decoder can tell you when the license will expire - this date includes the one-month grace past contract end we always give; it's the date that the apps will actually stop working.  Also, notice that this is a single-app license, as revealed by its precedence.  Finally, notice that we have specified the `-v` command-line flag so as to get additional information printed about the license (the Package UUID, the list of census codes for licensed machines, and the IDs of the certificate group in each license).

```
$ adobe-license-decoder -v isolated-photoshop/
 Preconditioning data for npdId: ZGQzMjhhY2MtZTE2Yy00NTI0LTgzOWItZGRkMDUwNTIzNGU0
     Package UUID: dd328acc-e16c-4524-839b-ddd0505234e4
     License type: FRL Isolated (2 codes)
     License codes: BB7BAC-WXJ2KG-366ZHJ, BBEFWI-B79KPQ-DUIEZI
     License expiry date: 2021-11-04
     Precedence: 80 (CC Single App)
 Application Licenses:
  1: App ID: Bridge1, Certificate Group: 2018072004
  2: App ID: Photoshop1, Certificate Group: 2018072004
$ sudo adobe-licensing-toolkit -p -i -f isolated-photoshop/ngl-preconditioning-data.json 
 Adobe Licensing Toolkit (1.1.0.91)
 Operation Successfully Completed
$ adobe-license-decoder -v
License files for npdId: ZGQzMjhhY2MtZTE2Yy00NTI0LTgzOWItZGRkMDUwNTIzNGU0:
    Package UUID: dd328acc-e16c-4524-839b-ddd0505234e4
    License type: FRL Isolated (2 codes)
    License codes: BB7BAC-WXJ2KG-366ZHJ, BBEFWI-B79KPQ-DUIEZI
    License expiry date: 2021-11-04
    Precedence: 80 (CC Single App)
Filenames (shown with '...' where the npdId appears):
 1: QnJpZGdlMXt9MjAxODA3MjAwNA-...-80.operatingconfig
    App ID: Bridge1, Certificate Group: 2018072004
    Install date: 2020-12-27 21:01:40 -08:00
 2: UGhvdG9zaG9wMXt9MjAxODA3MjAwNA-...-80.operatingconfig
    App ID: Photoshop1, Certificate Group: 2018072004
    Install date: 2020-12-27 21:01:39 -08:00
```

Finally, let's look at a run where we have installed a LAN package on top of the Isolated package.  (As above, we run the decoder over the package, then we install it, then we run the decoder to see what license files are on the machine.).  Because the LAN package and the Isolated package are both single-app packages, their licenses have the same precedence, so where there are two license files for the same application (in this case, Bridge), the LAN package will win because it has the later installation date.  It's in situations like these - where customers have installed two different packages on top of each other, that the decoder tool can really come in handy in understanding what's happened and in getting it fixed.

```
$ adobe-license-decoder lan-illustrator
Preconditioning data for npdId: OTUzZTViZWYtYWJmMy00NGUxLWFjYjUtZmZhN2MyMDY4YjQx
    License type: FRL LAN (server: https://test:123)
    License expiry date: controlled by server
    Precedence: 80 (CC Single App)
Application Licenses:
 1: App ID: Bridge1
 2: App ID: Illustrator1
$ sudo adobe-licensing-toolkit -p -i -f lan-illustrator/ngl-preconditioning-data.json 
Adobe Licensing Toolkit (1.1.0.91)
Operation Successfully Completed
$ adobe-license-decoder
License files for npdId: OTUzZTViZWYtYWJmMy00NGUxLWFjYjUtZmZhN2MyMDY4YjQx:
    License type: FRL LAN (server: https://test:123)
    License expiry date: controlled by server
    Precedence: 80 (CC Single App)
Filenames (shown with '...' where the npdId appears):
 1: QnJpZGdlMXt9MjAxODA3MjAwNA-...-80.operatingconfig
    App ID: Bridge1
    Install date: 2020-12-27 21:04:14 -08:00
 2: SWxsdXN0cmF0b3Ixe30yMDE4MDcyMDA0-...-80.operatingconfig
    App ID: Illustrator1
    Install date: 2020-12-27 21:04:13 -08:00
License files for npdId: ZGQzMjhhY2MtZTE2Yy00NTI0LTgzOWItZGRkMDUwNTIzNGU0:
    License type: FRL Isolated (2 codes)
    License expiry date: 2021-11-04
    Precedence: 80 (CC Single App)
Filenames (shown with '...' where the npdId appears):
 3: QnJpZGdlMXt9MjAxODA3MjAwNA-...-80.operatingconfig
    App ID: Bridge1
    Install date: 2020-12-27 21:01:40 -08:00
 4: UGhvdG9zaG9wMXt9MjAxODA3MjAwNA-...-80.operatingconfig
    App ID: Photoshop1
    Install date: 2020-12-27 21:01:39 -08:00
```

## Support

This tool is maintained by the Adobe DME Premium Onboarding team.  If you need support or just have questions about the `adobe-license-decoder`, please file an issue against this project.

## Contributing

Contributions are welcomed! Read the [Contributing Guide](./.github/CONTRIBUTING.md) for more information.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for more information.
