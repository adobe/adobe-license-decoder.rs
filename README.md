# adobe_license_decoder
This lightweight program is your secret decoder ring for FRL and SDL license files found on customer machines.  It can be used to analyze the content of the globally installed OperatingConfig (aka license) files and/or the preconditioning data in a license-only package.  It is a companion to the adobe-licensing-toolkit, which reads the cached licenses in a particular user account and can install/activate/deactivate/uninstall licenses globally.

## Usage
If invoked without any command-line arguments, the `adobe-license-decoder` will look for a globally installed Adobe/OperatingConfigs directory, and decode all the license files found in that directory.

If you have some other directory that you want it to look in for license files, just name that directory on the command line, as in:

```
adobe-license-decoder Downloads
```

If you have a license-only package, you can invoke the decoder on the preconditioning installation file in that package (which is the `.json`) file.  For example:

```
adobe-license-decoder Downloads/ngl-preconditioning-data.json
```

## Support
The `adobe-license-decoder` was written by Dan Brotsky ([dbrotsky@adobe.com](mailto:dbrotsky@adobe.com)) and you can contact him directly with questions or to report issues. The tool is being released to the field by James Lockman's DME Premium Onboarding team, who can provide in-field support if you are working with them on a project.
