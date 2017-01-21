# systemd-linter

[![Build Status](https://travis-ci.org/mackwic/systemd-linter.svg?branch=master)](https://travis-ci.org/mackwic/systemd-linter)
[![Windows build
status](https://ci.appveyor.com/api/projects/status/github/mackwic/systemd-linter?svg=true)](https://ci.appveyor.com/project/mackwic/systemd-linter)
[![Crates.io](https://img.shields.io/crates/v/systemd-linter.svg?maxAge=2592000)](https://crates.io/crates/systemd-linter)
[![Crates.io](https://img.shields.io/crates/l/systemd-linter.svg?maxAge=2592000)](https://github.com/mackwic/systemd-linter/blob/master/LICENSE)
[![Github Releases](https://img.shields.io/github/downloads/mackwic/systemd-linter/latest/total.svg)](https://github.com/mackwic/systemd-linter/releases/latest)
![status badge](https://img.shields.io/badge/churn-in_development-yellow.svg?style=flat)

`systemd-linter` lint SystemD unit files from any platform: Linux, OSX, and Windows. Validate your unit files, apply industries best-practices, and avoid hairy pitfalls.

See [the Releases Page](https://github.com/mackwic/systemd-linter/releases/latest) to download binaries.

| Confidence Item | Status |
| ------ | ---|
| False negatives | ✅ Will correctly parse all valid files |
| False positives | ⚠️ May not reject an inadequate file |
| Syntax Linting | ❌ Will not check for indentation, trailing whitespace, etc. |
| Unknown Directives | ✅ Will correctly detect unknown directives or categories |
| Documentation pitfalls | 📝 Implementation in progress |
| Craftmanship | 📝 Implementation in progress |



