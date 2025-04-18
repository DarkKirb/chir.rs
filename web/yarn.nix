{ fetchurl, fetchgit, linkFarm, runCommand, gnutar }: rec {
  offline_cache = linkFarm "offline" packages;
  packages = [
    {
      name = "_csstools_postcss_sass___postcss_sass_5.1.1.tgz";
      path = fetchurl {
        name = "_csstools_postcss_sass___postcss_sass_5.1.1.tgz";
        url  = "https://registry.yarnpkg.com/@csstools/postcss-sass/-/postcss-sass-5.1.1.tgz";
        sha512 = "La7bgTcM6YwPBLqlaXg7lMLry82iLv1a+S1RmgvHq2mH2Zd57L2anjZvJC8ACUHWc4M9fXws93dq6gaK0kZyAw==";
      };
    }
    {
      name = "_csstools_sass_import_resolve___sass_import_resolve_1.0.0.tgz";
      path = fetchurl {
        name = "_csstools_sass_import_resolve___sass_import_resolve_1.0.0.tgz";
        url  = "https://registry.yarnpkg.com/@csstools/sass-import-resolve/-/sass-import-resolve-1.0.0.tgz";
        sha512 = "pH4KCsbtBLLe7eqUrw8brcuFO8IZlN36JjdKlOublibVdAIPHCzEnpBWOVUXK5sCf+DpBi8ZtuWtjF0srybdeA==";
      };
    }
    {
      name = "_esbuild_aix_ppc64___aix_ppc64_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_aix_ppc64___aix_ppc64_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/aix-ppc64/-/aix-ppc64-0.25.2.tgz";
        sha512 = "wCIboOL2yXZym2cgm6mlA742s9QeJ8DjGVaL39dLN4rRwrOgOyYSnOaFPhKZGLb2ngj4EyfAFjsNJwPXZvseag==";
      };
    }
    {
      name = "_esbuild_android_arm64___android_arm64_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_android_arm64___android_arm64_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/android-arm64/-/android-arm64-0.25.2.tgz";
        sha512 = "5ZAX5xOmTligeBaeNEPnPaeEuah53Id2tX4c2CVP3JaROTH+j4fnfHCkr1PjXMd78hMst+TlkfKcW/DlTq0i4w==";
      };
    }
    {
      name = "_esbuild_android_arm___android_arm_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_android_arm___android_arm_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/android-arm/-/android-arm-0.25.2.tgz";
        sha512 = "NQhH7jFstVY5x8CKbcfa166GoV0EFkaPkCKBQkdPJFvo5u+nGXLEH/ooniLb3QI8Fk58YAx7nsPLozUWfCBOJA==";
      };
    }
    {
      name = "_esbuild_android_x64___android_x64_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_android_x64___android_x64_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/android-x64/-/android-x64-0.25.2.tgz";
        sha512 = "Ffcx+nnma8Sge4jzddPHCZVRvIfQ0kMsUsCMcJRHkGJ1cDmhe4SsrYIjLUKn1xpHZybmOqCWwB0zQvsjdEHtkg==";
      };
    }
    {
      name = "_esbuild_darwin_arm64___darwin_arm64_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_darwin_arm64___darwin_arm64_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/darwin-arm64/-/darwin-arm64-0.25.2.tgz";
        sha512 = "MpM6LUVTXAzOvN4KbjzU/q5smzryuoNjlriAIx+06RpecwCkL9JpenNzpKd2YMzLJFOdPqBpuub6eVRP5IgiSA==";
      };
    }
    {
      name = "_esbuild_darwin_x64___darwin_x64_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_darwin_x64___darwin_x64_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/darwin-x64/-/darwin-x64-0.25.2.tgz";
        sha512 = "5eRPrTX7wFyuWe8FqEFPG2cU0+butQQVNcT4sVipqjLYQjjh8a8+vUTfgBKM88ObB85ahsnTwF7PSIt6PG+QkA==";
      };
    }
    {
      name = "_esbuild_freebsd_arm64___freebsd_arm64_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_freebsd_arm64___freebsd_arm64_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/freebsd-arm64/-/freebsd-arm64-0.25.2.tgz";
        sha512 = "mLwm4vXKiQ2UTSX4+ImyiPdiHjiZhIaE9QvC7sw0tZ6HoNMjYAqQpGyui5VRIi5sGd+uWq940gdCbY3VLvsO1w==";
      };
    }
    {
      name = "_esbuild_freebsd_x64___freebsd_x64_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_freebsd_x64___freebsd_x64_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/freebsd-x64/-/freebsd-x64-0.25.2.tgz";
        sha512 = "6qyyn6TjayJSwGpm8J9QYYGQcRgc90nmfdUb0O7pp1s4lTY+9D0H9O02v5JqGApUyiHOtkz6+1hZNvNtEhbwRQ==";
      };
    }
    {
      name = "_esbuild_linux_arm64___linux_arm64_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_linux_arm64___linux_arm64_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/linux-arm64/-/linux-arm64-0.25.2.tgz";
        sha512 = "gq/sjLsOyMT19I8obBISvhoYiZIAaGF8JpeXu1u8yPv8BE5HlWYobmlsfijFIZ9hIVGYkbdFhEqC0NvM4kNO0g==";
      };
    }
    {
      name = "_esbuild_linux_arm___linux_arm_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_linux_arm___linux_arm_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/linux-arm/-/linux-arm-0.25.2.tgz";
        sha512 = "UHBRgJcmjJv5oeQF8EpTRZs/1knq6loLxTsjc3nxO9eXAPDLcWW55flrMVc97qFPbmZP31ta1AZVUKQzKTzb0g==";
      };
    }
    {
      name = "_esbuild_linux_ia32___linux_ia32_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_linux_ia32___linux_ia32_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/linux-ia32/-/linux-ia32-0.25.2.tgz";
        sha512 = "bBYCv9obgW2cBP+2ZWfjYTU+f5cxRoGGQ5SeDbYdFCAZpYWrfjjfYwvUpP8MlKbP0nwZ5gyOU/0aUzZ5HWPuvQ==";
      };
    }
    {
      name = "_esbuild_linux_loong64___linux_loong64_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_linux_loong64___linux_loong64_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/linux-loong64/-/linux-loong64-0.25.2.tgz";
        sha512 = "SHNGiKtvnU2dBlM5D8CXRFdd+6etgZ9dXfaPCeJtz+37PIUlixvlIhI23L5khKXs3DIzAn9V8v+qb1TRKrgT5w==";
      };
    }
    {
      name = "_esbuild_linux_mips64el___linux_mips64el_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_linux_mips64el___linux_mips64el_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/linux-mips64el/-/linux-mips64el-0.25.2.tgz";
        sha512 = "hDDRlzE6rPeoj+5fsADqdUZl1OzqDYow4TB4Y/3PlKBD0ph1e6uPHzIQcv2Z65u2K0kpeByIyAjCmjn1hJgG0Q==";
      };
    }
    {
      name = "_esbuild_linux_ppc64___linux_ppc64_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_linux_ppc64___linux_ppc64_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/linux-ppc64/-/linux-ppc64-0.25.2.tgz";
        sha512 = "tsHu2RRSWzipmUi9UBDEzc0nLc4HtpZEI5Ba+Omms5456x5WaNuiG3u7xh5AO6sipnJ9r4cRWQB2tUjPyIkc6g==";
      };
    }
    {
      name = "_esbuild_linux_riscv64___linux_riscv64_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_linux_riscv64___linux_riscv64_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/linux-riscv64/-/linux-riscv64-0.25.2.tgz";
        sha512 = "k4LtpgV7NJQOml/10uPU0s4SAXGnowi5qBSjaLWMojNCUICNu7TshqHLAEbkBdAszL5TabfvQ48kK84hyFzjnw==";
      };
    }
    {
      name = "_esbuild_linux_s390x___linux_s390x_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_linux_s390x___linux_s390x_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/linux-s390x/-/linux-s390x-0.25.2.tgz";
        sha512 = "GRa4IshOdvKY7M/rDpRR3gkiTNp34M0eLTaC1a08gNrh4u488aPhuZOCpkF6+2wl3zAN7L7XIpOFBhnaE3/Q8Q==";
      };
    }
    {
      name = "_esbuild_linux_x64___linux_x64_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_linux_x64___linux_x64_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/linux-x64/-/linux-x64-0.25.2.tgz";
        sha512 = "QInHERlqpTTZ4FRB0fROQWXcYRD64lAoiegezDunLpalZMjcUcld3YzZmVJ2H/Cp0wJRZ8Xtjtj0cEHhYc/uUg==";
      };
    }
    {
      name = "_esbuild_netbsd_arm64___netbsd_arm64_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_netbsd_arm64___netbsd_arm64_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/netbsd-arm64/-/netbsd-arm64-0.25.2.tgz";
        sha512 = "talAIBoY5M8vHc6EeI2WW9d/CkiO9MQJ0IOWX8hrLhxGbro/vBXJvaQXefW2cP0z0nQVTdQ/eNyGFV1GSKrxfw==";
      };
    }
    {
      name = "_esbuild_netbsd_x64___netbsd_x64_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_netbsd_x64___netbsd_x64_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/netbsd-x64/-/netbsd-x64-0.25.2.tgz";
        sha512 = "voZT9Z+tpOxrvfKFyfDYPc4DO4rk06qamv1a/fkuzHpiVBMOhpjK+vBmWM8J1eiB3OLSMFYNaOaBNLXGChf5tg==";
      };
    }
    {
      name = "_esbuild_openbsd_arm64___openbsd_arm64_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_openbsd_arm64___openbsd_arm64_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/openbsd-arm64/-/openbsd-arm64-0.25.2.tgz";
        sha512 = "dcXYOC6NXOqcykeDlwId9kB6OkPUxOEqU+rkrYVqJbK2hagWOMrsTGsMr8+rW02M+d5Op5NNlgMmjzecaRf7Tg==";
      };
    }
    {
      name = "_esbuild_openbsd_x64___openbsd_x64_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_openbsd_x64___openbsd_x64_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/openbsd-x64/-/openbsd-x64-0.25.2.tgz";
        sha512 = "t/TkWwahkH0Tsgoq1Ju7QfgGhArkGLkF1uYz8nQS/PPFlXbP5YgRpqQR3ARRiC2iXoLTWFxc6DJMSK10dVXluw==";
      };
    }
    {
      name = "_esbuild_sunos_x64___sunos_x64_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_sunos_x64___sunos_x64_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/sunos-x64/-/sunos-x64-0.25.2.tgz";
        sha512 = "cfZH1co2+imVdWCjd+D1gf9NjkchVhhdpgb1q5y6Hcv9TP6Zi9ZG/beI3ig8TvwT9lH9dlxLq5MQBBgwuj4xvA==";
      };
    }
    {
      name = "_esbuild_win32_arm64___win32_arm64_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_win32_arm64___win32_arm64_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/win32-arm64/-/win32-arm64-0.25.2.tgz";
        sha512 = "7Loyjh+D/Nx/sOTzV8vfbB3GJuHdOQyrOryFdZvPHLf42Tk9ivBU5Aedi7iyX+x6rbn2Mh68T4qq1SDqJBQO5Q==";
      };
    }
    {
      name = "_esbuild_win32_ia32___win32_ia32_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_win32_ia32___win32_ia32_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/win32-ia32/-/win32-ia32-0.25.2.tgz";
        sha512 = "WRJgsz9un0nqZJ4MfhabxaD9Ft8KioqU3JMinOTvobbX6MOSUigSBlogP8QB3uxpJDsFS6yN+3FDBdqE5lg9kg==";
      };
    }
    {
      name = "_esbuild_win32_x64___win32_x64_0.25.2.tgz";
      path = fetchurl {
        name = "_esbuild_win32_x64___win32_x64_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/@esbuild/win32-x64/-/win32-x64-0.25.2.tgz";
        sha512 = "kM3HKb16VIXZyIeVrM1ygYmZBKybX8N4p754bw390wGO3Tf2j4L2/WYL+4suWujpgf6GBYs3jv7TyUivdd05JA==";
      };
    }
    {
      name = "_parcel_watcher_android_arm64___watcher_android_arm64_2.5.1.tgz";
      path = fetchurl {
        name = "_parcel_watcher_android_arm64___watcher_android_arm64_2.5.1.tgz";
        url  = "https://registry.yarnpkg.com/@parcel/watcher-android-arm64/-/watcher-android-arm64-2.5.1.tgz";
        sha512 = "KF8+j9nNbUN8vzOFDpRMsaKBHZ/mcjEjMToVMJOhTozkDonQFFrRcfdLWn6yWKCmJKmdVxSgHiYvTCef4/qcBA==";
      };
    }
    {
      name = "_parcel_watcher_darwin_arm64___watcher_darwin_arm64_2.5.1.tgz";
      path = fetchurl {
        name = "_parcel_watcher_darwin_arm64___watcher_darwin_arm64_2.5.1.tgz";
        url  = "https://registry.yarnpkg.com/@parcel/watcher-darwin-arm64/-/watcher-darwin-arm64-2.5.1.tgz";
        sha512 = "eAzPv5osDmZyBhou8PoF4i6RQXAfeKL9tjb3QzYuccXFMQU0ruIc/POh30ePnaOyD1UXdlKguHBmsTs53tVoPw==";
      };
    }
    {
      name = "_parcel_watcher_darwin_x64___watcher_darwin_x64_2.5.1.tgz";
      path = fetchurl {
        name = "_parcel_watcher_darwin_x64___watcher_darwin_x64_2.5.1.tgz";
        url  = "https://registry.yarnpkg.com/@parcel/watcher-darwin-x64/-/watcher-darwin-x64-2.5.1.tgz";
        sha512 = "1ZXDthrnNmwv10A0/3AJNZ9JGlzrF82i3gNQcWOzd7nJ8aj+ILyW1MTxVk35Db0u91oD5Nlk9MBiujMlwmeXZg==";
      };
    }
    {
      name = "_parcel_watcher_freebsd_x64___watcher_freebsd_x64_2.5.1.tgz";
      path = fetchurl {
        name = "_parcel_watcher_freebsd_x64___watcher_freebsd_x64_2.5.1.tgz";
        url  = "https://registry.yarnpkg.com/@parcel/watcher-freebsd-x64/-/watcher-freebsd-x64-2.5.1.tgz";
        sha512 = "SI4eljM7Flp9yPuKi8W0ird8TI/JK6CSxju3NojVI6BjHsTyK7zxA9urjVjEKJ5MBYC+bLmMcbAWlZ+rFkLpJQ==";
      };
    }
    {
      name = "_parcel_watcher_linux_arm_glibc___watcher_linux_arm_glibc_2.5.1.tgz";
      path = fetchurl {
        name = "_parcel_watcher_linux_arm_glibc___watcher_linux_arm_glibc_2.5.1.tgz";
        url  = "https://registry.yarnpkg.com/@parcel/watcher-linux-arm-glibc/-/watcher-linux-arm-glibc-2.5.1.tgz";
        sha512 = "RCdZlEyTs8geyBkkcnPWvtXLY44BCeZKmGYRtSgtwwnHR4dxfHRG3gR99XdMEdQ7KeiDdasJwwvNSF5jKtDwdA==";
      };
    }
    {
      name = "_parcel_watcher_linux_arm_musl___watcher_linux_arm_musl_2.5.1.tgz";
      path = fetchurl {
        name = "_parcel_watcher_linux_arm_musl___watcher_linux_arm_musl_2.5.1.tgz";
        url  = "https://registry.yarnpkg.com/@parcel/watcher-linux-arm-musl/-/watcher-linux-arm-musl-2.5.1.tgz";
        sha512 = "6E+m/Mm1t1yhB8X412stiKFG3XykmgdIOqhjWj+VL8oHkKABfu/gjFj8DvLrYVHSBNC+/u5PeNrujiSQ1zwd1Q==";
      };
    }
    {
      name = "_parcel_watcher_linux_arm64_glibc___watcher_linux_arm64_glibc_2.5.1.tgz";
      path = fetchurl {
        name = "_parcel_watcher_linux_arm64_glibc___watcher_linux_arm64_glibc_2.5.1.tgz";
        url  = "https://registry.yarnpkg.com/@parcel/watcher-linux-arm64-glibc/-/watcher-linux-arm64-glibc-2.5.1.tgz";
        sha512 = "LrGp+f02yU3BN9A+DGuY3v3bmnFUggAITBGriZHUREfNEzZh/GO06FF5u2kx8x+GBEUYfyTGamol4j3m9ANe8w==";
      };
    }
    {
      name = "_parcel_watcher_linux_arm64_musl___watcher_linux_arm64_musl_2.5.1.tgz";
      path = fetchurl {
        name = "_parcel_watcher_linux_arm64_musl___watcher_linux_arm64_musl_2.5.1.tgz";
        url  = "https://registry.yarnpkg.com/@parcel/watcher-linux-arm64-musl/-/watcher-linux-arm64-musl-2.5.1.tgz";
        sha512 = "cFOjABi92pMYRXS7AcQv9/M1YuKRw8SZniCDw0ssQb/noPkRzA+HBDkwmyOJYp5wXcsTrhxO0zq1U11cK9jsFg==";
      };
    }
    {
      name = "_parcel_watcher_linux_x64_glibc___watcher_linux_x64_glibc_2.5.1.tgz";
      path = fetchurl {
        name = "_parcel_watcher_linux_x64_glibc___watcher_linux_x64_glibc_2.5.1.tgz";
        url  = "https://registry.yarnpkg.com/@parcel/watcher-linux-x64-glibc/-/watcher-linux-x64-glibc-2.5.1.tgz";
        sha512 = "GcESn8NZySmfwlTsIur+49yDqSny2IhPeZfXunQi48DMugKeZ7uy1FX83pO0X22sHntJ4Ub+9k34XQCX+oHt2A==";
      };
    }
    {
      name = "_parcel_watcher_linux_x64_musl___watcher_linux_x64_musl_2.5.1.tgz";
      path = fetchurl {
        name = "_parcel_watcher_linux_x64_musl___watcher_linux_x64_musl_2.5.1.tgz";
        url  = "https://registry.yarnpkg.com/@parcel/watcher-linux-x64-musl/-/watcher-linux-x64-musl-2.5.1.tgz";
        sha512 = "n0E2EQbatQ3bXhcH2D1XIAANAcTZkQICBPVaxMeaCVBtOpBZpWJuf7LwyWPSBDITb7In8mqQgJ7gH8CILCURXg==";
      };
    }
    {
      name = "_parcel_watcher_win32_arm64___watcher_win32_arm64_2.5.1.tgz";
      path = fetchurl {
        name = "_parcel_watcher_win32_arm64___watcher_win32_arm64_2.5.1.tgz";
        url  = "https://registry.yarnpkg.com/@parcel/watcher-win32-arm64/-/watcher-win32-arm64-2.5.1.tgz";
        sha512 = "RFzklRvmc3PkjKjry3hLF9wD7ppR4AKcWNzH7kXR7GUe0Igb3Nz8fyPwtZCSquGrhU5HhUNDr/mKBqj7tqA2Vw==";
      };
    }
    {
      name = "_parcel_watcher_win32_ia32___watcher_win32_ia32_2.5.1.tgz";
      path = fetchurl {
        name = "_parcel_watcher_win32_ia32___watcher_win32_ia32_2.5.1.tgz";
        url  = "https://registry.yarnpkg.com/@parcel/watcher-win32-ia32/-/watcher-win32-ia32-2.5.1.tgz";
        sha512 = "c2KkcVN+NJmuA7CGlaGD1qJh1cLfDnQsHjE89E60vUEMlqduHGCdCLJCID5geFVM0dOtA3ZiIO8BoEQmzQVfpQ==";
      };
    }
    {
      name = "_parcel_watcher_win32_x64___watcher_win32_x64_2.5.1.tgz";
      path = fetchurl {
        name = "_parcel_watcher_win32_x64___watcher_win32_x64_2.5.1.tgz";
        url  = "https://registry.yarnpkg.com/@parcel/watcher-win32-x64/-/watcher-win32-x64-2.5.1.tgz";
        sha512 = "9lHBdJITeNR++EvSQVUcaZoWupyHfXe1jZvGZ06O/5MflPcuPLtEphScIBL+AiCWBO46tDSHzWyD0uDmmZqsgA==";
      };
    }
    {
      name = "_parcel_watcher___watcher_2.5.1.tgz";
      path = fetchurl {
        name = "_parcel_watcher___watcher_2.5.1.tgz";
        url  = "https://registry.yarnpkg.com/@parcel/watcher/-/watcher-2.5.1.tgz";
        sha512 = "dfUnCxiN9H4ap84DvD2ubjw+3vUNpstxa0TneY/Paat8a3R4uQZDLSvWjmznAY/DoahqTHl9V46HF/Zs3F29pg==";
      };
    }
    {
      name = "_picocss_pico___pico_2.1.1.tgz";
      path = fetchurl {
        name = "_picocss_pico___pico_2.1.1.tgz";
        url  = "https://registry.yarnpkg.com/@picocss/pico/-/pico-2.1.1.tgz";
        sha512 = "kIDugA7Ps4U+2BHxiNHmvgPIQDWPDU4IeU6TNRdvXQM1uZX+FibqDQT2xUOnnO2yq/LUHcwnGlu1hvf4KfXnMg==";
      };
    }
    {
      name = "_trysound_sax___sax_0.2.0.tgz";
      path = fetchurl {
        name = "_trysound_sax___sax_0.2.0.tgz";
        url  = "https://registry.yarnpkg.com/@trysound/sax/-/sax-0.2.0.tgz";
        sha512 = "L7z9BgrNEcYyUYtF+HaEfiS5ebkh9jXqbszz7pC0hRBPaatV0XjSD3+eHrpqFemQfgwiFF0QPIarnIihIDn7OA==";
      };
    }
    {
      name = "ansi_regex___ansi_regex_5.0.1.tgz";
      path = fetchurl {
        name = "ansi_regex___ansi_regex_5.0.1.tgz";
        url  = "https://registry.yarnpkg.com/ansi-regex/-/ansi-regex-5.0.1.tgz";
        sha512 = "quJQXlTSUGL2LH9SUXo8VwsY4soanhgo6LNSm84E1LBcE8s3O0wpdiRzyR9z/ZZJMlMWv37qOOb9pdJlMUEKFQ==";
      };
    }
    {
      name = "ansi_styles___ansi_styles_4.3.0.tgz";
      path = fetchurl {
        name = "ansi_styles___ansi_styles_4.3.0.tgz";
        url  = "https://registry.yarnpkg.com/ansi-styles/-/ansi-styles-4.3.0.tgz";
        sha512 = "zbB9rCJAT1rbjiVDb2hqKFHNYLxgtk8NURxZ3IZwD3F6NtxbXZQCnnSi1Lkx+IDohdPlFp222wVALIheZJQSEg==";
      };
    }
    {
      name = "anymatch___anymatch_3.1.3.tgz";
      path = fetchurl {
        name = "anymatch___anymatch_3.1.3.tgz";
        url  = "https://registry.yarnpkg.com/anymatch/-/anymatch-3.1.3.tgz";
        sha512 = "KMReFUr0B4t+D+OBkjR3KYqvocp2XaSzO55UcB6mgQMd3KbcE+mWTyvVV7D/zsdEbNnV6acZUutkiHQXvTr1Rw==";
      };
    }
    {
      name = "autoprefixer___autoprefixer_10.4.21.tgz";
      path = fetchurl {
        name = "autoprefixer___autoprefixer_10.4.21.tgz";
        url  = "https://registry.yarnpkg.com/autoprefixer/-/autoprefixer-10.4.21.tgz";
        sha512 = "O+A6LWV5LDHSJD3LjHYoNi4VLsj/Whi7k6zG12xTYaU4cQ8oxQGckXNX8cRHK5yOZ/ppVHe0ZBXGzSV9jXdVbQ==";
      };
    }
    {
      name = "binary_extensions___binary_extensions_2.3.0.tgz";
      path = fetchurl {
        name = "binary_extensions___binary_extensions_2.3.0.tgz";
        url  = "https://registry.yarnpkg.com/binary-extensions/-/binary-extensions-2.3.0.tgz";
        sha512 = "Ceh+7ox5qe7LJuLHoY0feh3pHuUDHAcRUeyL2VYghZwfpkNIy/+8Ocg0a3UuSoYzavmylwuLWQOf3hl0jjMMIw==";
      };
    }
    {
      name = "boolbase___boolbase_1.0.0.tgz";
      path = fetchurl {
        name = "boolbase___boolbase_1.0.0.tgz";
        url  = "https://registry.yarnpkg.com/boolbase/-/boolbase-1.0.0.tgz";
        sha512 = "JZOSA7Mo9sNGB8+UjSgzdLtokWAky1zbztM3WRLCbZ70/3cTANmQmOdR7y2g+J0e2WXywy1yS468tY+IruqEww==";
      };
    }
    {
      name = "braces___braces_3.0.3.tgz";
      path = fetchurl {
        name = "braces___braces_3.0.3.tgz";
        url  = "https://registry.yarnpkg.com/braces/-/braces-3.0.3.tgz";
        sha512 = "yQbXgO/OSZVD2IsiLlro+7Hf6Q18EJrKSEsdoMzKePKXct3gvD8oLcOQdIzGupr5Fj+EDe8gO/lxc1BzfMpxvA==";
      };
    }
    {
      name = "browserslist___browserslist_4.24.4.tgz";
      path = fetchurl {
        name = "browserslist___browserslist_4.24.4.tgz";
        url  = "https://registry.yarnpkg.com/browserslist/-/browserslist-4.24.4.tgz";
        sha512 = "KDi1Ny1gSePi1vm0q4oxSF8b4DR44GF4BbmS2YdhPLOEqd8pDviZOGH/GsmRwoWJ2+5Lr085X7naowMwKHDG1A==";
      };
    }
    {
      name = "caniuse_api___caniuse_api_3.0.0.tgz";
      path = fetchurl {
        name = "caniuse_api___caniuse_api_3.0.0.tgz";
        url  = "https://registry.yarnpkg.com/caniuse-api/-/caniuse-api-3.0.0.tgz";
        sha512 = "bsTwuIg/BZZK/vreVTYYbSWoe2F+71P7K5QGEX+pT250DZbfU1MQ5prOKpPR+LL6uWKK3KMwMCAS74QB3Um1uw==";
      };
    }
    {
      name = "caniuse_lite___caniuse_lite_1.0.30001714.tgz";
      path = fetchurl {
        name = "caniuse_lite___caniuse_lite_1.0.30001714.tgz";
        url  = "https://registry.yarnpkg.com/caniuse-lite/-/caniuse-lite-1.0.30001714.tgz";
        sha512 = "mtgapdwDLSSBnCI3JokHM7oEQBLxiJKVRtg10AxM1AyeiKcM96f0Mkbqeq+1AbiCtvMcHRulAAEMu693JrSWqg==";
      };
    }
    {
      name = "chokidar___chokidar_3.6.0.tgz";
      path = fetchurl {
        name = "chokidar___chokidar_3.6.0.tgz";
        url  = "https://registry.yarnpkg.com/chokidar/-/chokidar-3.6.0.tgz";
        sha512 = "7VT13fmjotKpGipCW9JEQAusEPE+Ei8nl6/g4FBAmIm0GOOLMua9NDDo/DWp0ZAxCr3cPq5ZpBqmPAQgDda2Pw==";
      };
    }
    {
      name = "chokidar___chokidar_4.0.3.tgz";
      path = fetchurl {
        name = "chokidar___chokidar_4.0.3.tgz";
        url  = "https://registry.yarnpkg.com/chokidar/-/chokidar-4.0.3.tgz";
        sha512 = "Qgzu8kfBvo+cA4962jnP1KkS6Dop5NS6g7R5LFYJr4b8Ub94PPQXUksCw9PvXoeXPRRddRNC5C1JQUR2SMGtnA==";
      };
    }
    {
      name = "cliui___cliui_8.0.1.tgz";
      path = fetchurl {
        name = "cliui___cliui_8.0.1.tgz";
        url  = "https://registry.yarnpkg.com/cliui/-/cliui-8.0.1.tgz";
        sha512 = "BSeNnyus75C4//NQ9gQt1/csTXyo/8Sb+afLAkzAptFuMsod9HFokGNudZpi/oQV73hnVK+sR+5PVRMd+Dr7YQ==";
      };
    }
    {
      name = "color_convert___color_convert_2.0.1.tgz";
      path = fetchurl {
        name = "color_convert___color_convert_2.0.1.tgz";
        url  = "https://registry.yarnpkg.com/color-convert/-/color-convert-2.0.1.tgz";
        sha512 = "RRECPsj7iu/xb5oKYcsFHSppFNnsj/52OVTRKb4zP5onXwVF3zVmmToNcOfGC+CRDpfK/U584fMg38ZHCaElKQ==";
      };
    }
    {
      name = "color_name___color_name_1.1.4.tgz";
      path = fetchurl {
        name = "color_name___color_name_1.1.4.tgz";
        url  = "https://registry.yarnpkg.com/color-name/-/color-name-1.1.4.tgz";
        sha512 = "dOy+3AuW3a2wNbZHIuMZpTcgjGuLU/uBL/ubcZF9OXbDo8ff4O8yVp5Bf0efS8uEoYo5q4Fx7dY9OgQGXgAsQA==";
      };
    }
    {
      name = "colord___colord_2.9.3.tgz";
      path = fetchurl {
        name = "colord___colord_2.9.3.tgz";
        url  = "https://registry.yarnpkg.com/colord/-/colord-2.9.3.tgz";
        sha512 = "jeC1axXpnb0/2nn/Y1LPuLdgXBLH7aDcHu4KEKfqw3CUhX7ZpfBSlPKyqXE6btIgEzfWtrX3/tyBCaCvXvMkOw==";
      };
    }
    {
      name = "commander___commander_7.2.0.tgz";
      path = fetchurl {
        name = "commander___commander_7.2.0.tgz";
        url  = "https://registry.yarnpkg.com/commander/-/commander-7.2.0.tgz";
        sha512 = "QrWXB+ZQSVPmIWIhtEO9H+gwHaMGYiF5ChvoJ+K9ZGHG/sVsa6yiesAD1GC/x46sET00Xlwo1u49RVVVzvcSkw==";
      };
    }
    {
      name = "css_declaration_sorter___css_declaration_sorter_7.2.0.tgz";
      path = fetchurl {
        name = "css_declaration_sorter___css_declaration_sorter_7.2.0.tgz";
        url  = "https://registry.yarnpkg.com/css-declaration-sorter/-/css-declaration-sorter-7.2.0.tgz";
        sha512 = "h70rUM+3PNFuaBDTLe8wF/cdWu+dOZmb7pJt8Z2sedYbAcQVQV/tEchueg3GWxwqS0cxtbxmaHEdkNACqcvsow==";
      };
    }
    {
      name = "css_select___css_select_5.1.0.tgz";
      path = fetchurl {
        name = "css_select___css_select_5.1.0.tgz";
        url  = "https://registry.yarnpkg.com/css-select/-/css-select-5.1.0.tgz";
        sha512 = "nwoRF1rvRRnnCqqY7updORDsuqKzqYJ28+oSMaJMMgOauh3fvwHqMS7EZpIPqK8GL+g9mKxF1vP/ZjSeNjEVHg==";
      };
    }
    {
      name = "css_tree___css_tree_2.3.1.tgz";
      path = fetchurl {
        name = "css_tree___css_tree_2.3.1.tgz";
        url  = "https://registry.yarnpkg.com/css-tree/-/css-tree-2.3.1.tgz";
        sha512 = "6Fv1DV/TYw//QF5IzQdqsNDjx/wc8TrMBZsqjL9eW01tWb7R7k/mq+/VXfJCl7SoD5emsJop9cOByJZfs8hYIw==";
      };
    }
    {
      name = "css_tree___css_tree_2.2.1.tgz";
      path = fetchurl {
        name = "css_tree___css_tree_2.2.1.tgz";
        url  = "https://registry.yarnpkg.com/css-tree/-/css-tree-2.2.1.tgz";
        sha512 = "OA0mILzGc1kCOCSJerOeqDxDQ4HOh+G8NbOJFOTgOCzpw7fCBubk0fEyxp8AgOL/jvLgYA/uV0cMbe43ElF1JA==";
      };
    }
    {
      name = "css_what___css_what_6.1.0.tgz";
      path = fetchurl {
        name = "css_what___css_what_6.1.0.tgz";
        url  = "https://registry.yarnpkg.com/css-what/-/css-what-6.1.0.tgz";
        sha512 = "HTUrgRJ7r4dsZKU6GjmpfRK1O76h97Z8MfS1G0FozR+oF2kG6Vfe8JE6zwrkbxigziPHinCJ+gCPjA9EaBDtRw==";
      };
    }
    {
      name = "cssesc___cssesc_3.0.0.tgz";
      path = fetchurl {
        name = "cssesc___cssesc_3.0.0.tgz";
        url  = "https://registry.yarnpkg.com/cssesc/-/cssesc-3.0.0.tgz";
        sha512 = "/Tb/JcjK111nNScGob5MNtsntNM1aCNUDipB/TkwZFhyDrrE47SOx/18wF2bbjgc3ZzCSKW1T5nt5EbFoAz/Vg==";
      };
    }
    {
      name = "cssnano_preset_advanced___cssnano_preset_advanced_7.0.6.tgz";
      path = fetchurl {
        name = "cssnano_preset_advanced___cssnano_preset_advanced_7.0.6.tgz";
        url  = "https://registry.yarnpkg.com/cssnano-preset-advanced/-/cssnano-preset-advanced-7.0.6.tgz";
        sha512 = "wk/YPSv965EjpPNEGteiXZ32BKilJcYNnX4EGUd/AriVGgHL/y59uaWVJ/ZDx69jCNUrmwiBzioCV+SG5wk3PQ==";
      };
    }
    {
      name = "cssnano_preset_default___cssnano_preset_default_7.0.6.tgz";
      path = fetchurl {
        name = "cssnano_preset_default___cssnano_preset_default_7.0.6.tgz";
        url  = "https://registry.yarnpkg.com/cssnano-preset-default/-/cssnano-preset-default-7.0.6.tgz";
        sha512 = "ZzrgYupYxEvdGGuqL+JKOY70s7+saoNlHSCK/OGn1vB2pQK8KSET8jvenzItcY+kA7NoWvfbb/YhlzuzNKjOhQ==";
      };
    }
    {
      name = "cssnano_utils___cssnano_utils_5.0.0.tgz";
      path = fetchurl {
        name = "cssnano_utils___cssnano_utils_5.0.0.tgz";
        url  = "https://registry.yarnpkg.com/cssnano-utils/-/cssnano-utils-5.0.0.tgz";
        sha512 = "Uij0Xdxc24L6SirFr25MlwC2rCFX6scyUmuKpzI+JQ7cyqDEwD42fJ0xfB3yLfOnRDU5LKGgjQ9FA6LYh76GWQ==";
      };
    }
    {
      name = "cssnano___cssnano_7.0.6.tgz";
      path = fetchurl {
        name = "cssnano___cssnano_7.0.6.tgz";
        url  = "https://registry.yarnpkg.com/cssnano/-/cssnano-7.0.6.tgz";
        sha512 = "54woqx8SCbp8HwvNZYn68ZFAepuouZW4lTwiMVnBErM3VkO7/Sd4oTOt3Zz3bPx3kxQ36aISppyXj2Md4lg8bw==";
      };
    }
    {
      name = "csso___csso_5.0.5.tgz";
      path = fetchurl {
        name = "csso___csso_5.0.5.tgz";
        url  = "https://registry.yarnpkg.com/csso/-/csso-5.0.5.tgz";
        sha512 = "0LrrStPOdJj+SPCCrGhzryycLjwcgUSHBtxNA8aIDxf0GLsRh1cKYhB00Gd1lDOS4yGH69+SNn13+TWbVHETFQ==";
      };
    }
    {
      name = "dependency_graph___dependency_graph_1.0.0.tgz";
      path = fetchurl {
        name = "dependency_graph___dependency_graph_1.0.0.tgz";
        url  = "https://registry.yarnpkg.com/dependency-graph/-/dependency-graph-1.0.0.tgz";
        sha512 = "cW3gggJ28HZ/LExwxP2B++aiKxhJXMSIt9K48FOXQkm+vuG5gyatXnLsONRJdzO/7VfjDIiaOOa/bs4l464Lwg==";
      };
    }
    {
      name = "detect_libc___detect_libc_1.0.3.tgz";
      path = fetchurl {
        name = "detect_libc___detect_libc_1.0.3.tgz";
        url  = "https://registry.yarnpkg.com/detect-libc/-/detect-libc-1.0.3.tgz";
        sha512 = "pGjwhsmsp4kL2RTz08wcOlGN83otlqHeD/Z5T8GXZB+/YcpQ/dgo+lbU8ZsGxV0HIvqqxo9l7mqYwyYMD9bKDg==";
      };
    }
    {
      name = "dom_serializer___dom_serializer_2.0.0.tgz";
      path = fetchurl {
        name = "dom_serializer___dom_serializer_2.0.0.tgz";
        url  = "https://registry.yarnpkg.com/dom-serializer/-/dom-serializer-2.0.0.tgz";
        sha512 = "wIkAryiqt/nV5EQKqQpo3SToSOV9J0DnbJqwK7Wv/Trc92zIAYZ4FlMu+JPFW1DfGFt81ZTCGgDEabffXeLyJg==";
      };
    }
    {
      name = "domelementtype___domelementtype_2.3.0.tgz";
      path = fetchurl {
        name = "domelementtype___domelementtype_2.3.0.tgz";
        url  = "https://registry.yarnpkg.com/domelementtype/-/domelementtype-2.3.0.tgz";
        sha512 = "OLETBj6w0OsagBwdXnPdN0cnMfF9opN69co+7ZrbfPGrdpPVNBUj02spi6B1N7wChLQiPn4CSH/zJvXw56gmHw==";
      };
    }
    {
      name = "domhandler___domhandler_5.0.3.tgz";
      path = fetchurl {
        name = "domhandler___domhandler_5.0.3.tgz";
        url  = "https://registry.yarnpkg.com/domhandler/-/domhandler-5.0.3.tgz";
        sha512 = "cgwlv/1iFQiFnU96XXgROh8xTeetsnJiDsTc7TYCLFd9+/WNkIqPTxiM/8pSd8VIrhXGTf1Ny1q1hquVqDJB5w==";
      };
    }
    {
      name = "domutils___domutils_3.2.2.tgz";
      path = fetchurl {
        name = "domutils___domutils_3.2.2.tgz";
        url  = "https://registry.yarnpkg.com/domutils/-/domutils-3.2.2.tgz";
        sha512 = "6kZKyUajlDuqlHKVX1w7gyslj9MPIXzIFiz/rGu35uC1wMi+kMhQwGhl4lt9unC9Vb9INnY9Z3/ZA3+FhASLaw==";
      };
    }
    {
      name = "electron_to_chromium___electron_to_chromium_1.5.138.tgz";
      path = fetchurl {
        name = "electron_to_chromium___electron_to_chromium_1.5.138.tgz";
        url  = "https://registry.yarnpkg.com/electron-to-chromium/-/electron-to-chromium-1.5.138.tgz";
        sha512 = "FWlQc52z1dXqm+9cCJ2uyFgJkESd+16j6dBEjsgDNuHjBpuIzL8/lRc0uvh1k8RNI6waGo6tcy2DvwkTBJOLDg==";
      };
    }
    {
      name = "emoji_regex___emoji_regex_8.0.0.tgz";
      path = fetchurl {
        name = "emoji_regex___emoji_regex_8.0.0.tgz";
        url  = "https://registry.yarnpkg.com/emoji-regex/-/emoji-regex-8.0.0.tgz";
        sha512 = "MSjYzcWNOA0ewAHpz0MxpYFvwg6yjy1NG3xteoqz644VCo/RPgnr1/GGt+ic3iJTzQ8Eu3TdM14SawnVUmGE6A==";
      };
    }
    {
      name = "entities___entities_4.5.0.tgz";
      path = fetchurl {
        name = "entities___entities_4.5.0.tgz";
        url  = "https://registry.yarnpkg.com/entities/-/entities-4.5.0.tgz";
        sha512 = "V0hjH4dGPh9Ao5p0MoRY6BVqtwCjhz6vI5LT8AJ55H+4g9/4vbHx1I54fS0XuclLhDHArPQCiMjDxjaL8fPxhw==";
      };
    }
    {
      name = "esbuild_plugin_tsc___esbuild_plugin_tsc_0.5.0.tgz";
      path = fetchurl {
        name = "esbuild_plugin_tsc___esbuild_plugin_tsc_0.5.0.tgz";
        url  = "https://registry.yarnpkg.com/esbuild-plugin-tsc/-/esbuild-plugin-tsc-0.5.0.tgz";
        sha512 = "t9j90NnMhAGWzS0SKX3zNa/XeWcUanqKaFe36CSmXiB2nYFdaSzKY9pBZTvGV7NGImxE1BktOOlVpQyYnUTVGg==";
      };
    }
    {
      name = "esbuild___esbuild_0.25.2.tgz";
      path = fetchurl {
        name = "esbuild___esbuild_0.25.2.tgz";
        url  = "https://registry.yarnpkg.com/esbuild/-/esbuild-0.25.2.tgz";
        sha512 = "16854zccKPnC+toMywC+uKNeYSv+/eXkevRAfwRD/G9Cleq66m8XFIrigkbvauLLlCfDL45Q2cWegSg53gGBnQ==";
      };
    }
    {
      name = "escalade___escalade_3.2.0.tgz";
      path = fetchurl {
        name = "escalade___escalade_3.2.0.tgz";
        url  = "https://registry.yarnpkg.com/escalade/-/escalade-3.2.0.tgz";
        sha512 = "WUj2qlxaQtO4g6Pq5c29GTcWGDyd8itL8zTlipgECz3JesAiiOKotd8JU6otB3PACgG6xkJUyVhboMS+bje/jA==";
      };
    }
    {
      name = "fdir___fdir_6.4.3.tgz";
      path = fetchurl {
        name = "fdir___fdir_6.4.3.tgz";
        url  = "https://registry.yarnpkg.com/fdir/-/fdir-6.4.3.tgz";
        sha512 = "PMXmW2y1hDDfTSRc9gaXIuCCRpuoz3Kaz8cUelp3smouvfT632ozg2vrT6lJsHKKOF59YLbOGfAWGUcKEfRMQw==";
      };
    }
    {
      name = "fill_range___fill_range_7.1.1.tgz";
      path = fetchurl {
        name = "fill_range___fill_range_7.1.1.tgz";
        url  = "https://registry.yarnpkg.com/fill-range/-/fill-range-7.1.1.tgz";
        sha512 = "YsGpe3WHLK8ZYi4tWDg2Jy3ebRz2rXowDxnld4bkQB00cc/1Zw9AWnC0i9ztDJitivtQvaI9KaLyKrc+hBW0yg==";
      };
    }
    {
      name = "fraction.js___fraction.js_4.3.7.tgz";
      path = fetchurl {
        name = "fraction.js___fraction.js_4.3.7.tgz";
        url  = "https://registry.yarnpkg.com/fraction.js/-/fraction.js-4.3.7.tgz";
        sha512 = "ZsDfxO51wGAXREY55a7la9LScWpwv9RxIrYABrlvOFBlH/ShPnrtsXeuUIfXKKOVicNxQ+o8JTbJvjS4M89yew==";
      };
    }
    {
      name = "fs_extra___fs_extra_11.3.0.tgz";
      path = fetchurl {
        name = "fs_extra___fs_extra_11.3.0.tgz";
        url  = "https://registry.yarnpkg.com/fs-extra/-/fs-extra-11.3.0.tgz";
        sha512 = "Z4XaCL6dUDHfP/jT25jJKMmtxvuwbkrD1vNSMFlo9lNLY2c5FHYSQgHPRZUjAB26TpDEoW9HCOgplrdbaPV/ew==";
      };
    }
    {
      name = "fsevents___fsevents_2.3.3.tgz";
      path = fetchurl {
        name = "fsevents___fsevents_2.3.3.tgz";
        url  = "https://registry.yarnpkg.com/fsevents/-/fsevents-2.3.3.tgz";
        sha512 = "5xoDfX+fL7faATnagmWPpbFtwh/R77WmMMqqHGS65C3vvB0YHrgF+B1YmZ3441tMj5n63k0212XNoJwzlhffQw==";
      };
    }
    {
      name = "get_caller_file___get_caller_file_2.0.5.tgz";
      path = fetchurl {
        name = "get_caller_file___get_caller_file_2.0.5.tgz";
        url  = "https://registry.yarnpkg.com/get-caller-file/-/get-caller-file-2.0.5.tgz";
        sha512 = "DyFP3BM/3YHTQOCUL/w0OZHR0lpKeGrxotcHWcqNEdnltqFwXVfhEBQ94eIo34AfQpo0rGki4cyIiftY06h2Fg==";
      };
    }
    {
      name = "glob_parent___glob_parent_5.1.2.tgz";
      path = fetchurl {
        name = "glob_parent___glob_parent_5.1.2.tgz";
        url  = "https://registry.yarnpkg.com/glob-parent/-/glob-parent-5.1.2.tgz";
        sha512 = "AOIgSQCepiJYwP3ARnGx+5VnTu2HBYdzbGP45eLw1vr3zB3vZLeyed1sC9hnbcOc9/SrMyM5RPQrkGz4aS9Zow==";
      };
    }
    {
      name = "graceful_fs___graceful_fs_4.2.11.tgz";
      path = fetchurl {
        name = "graceful_fs___graceful_fs_4.2.11.tgz";
        url  = "https://registry.yarnpkg.com/graceful-fs/-/graceful-fs-4.2.11.tgz";
        sha512 = "RbJ5/jmFcNNCcDV5o9eTnBLJ/HszWV0P73bc+Ff4nS/rJj+YaS6IGyiOL0VoBYX+l1Wrl3k63h/KrH+nhJ0XvQ==";
      };
    }
    {
      name = "immutable___immutable_5.1.1.tgz";
      path = fetchurl {
        name = "immutable___immutable_5.1.1.tgz";
        url  = "https://registry.yarnpkg.com/immutable/-/immutable-5.1.1.tgz";
        sha512 = "3jatXi9ObIsPGr3N5hGw/vWWcTkq6hUYhpQz4k0wLC+owqWi/LiugIw9x0EdNZ2yGedKN/HzePiBvaJRXa0Ujg==";
      };
    }
    {
      name = "is_binary_path___is_binary_path_2.1.0.tgz";
      path = fetchurl {
        name = "is_binary_path___is_binary_path_2.1.0.tgz";
        url  = "https://registry.yarnpkg.com/is-binary-path/-/is-binary-path-2.1.0.tgz";
        sha512 = "ZMERYes6pDydyuGidse7OsHxtbI7WVeUEozgR/g7rd0xUimYNlvZRE/K2MgZTjWy725IfelLeVcEM97mmtRGXw==";
      };
    }
    {
      name = "is_extglob___is_extglob_2.1.1.tgz";
      path = fetchurl {
        name = "is_extglob___is_extglob_2.1.1.tgz";
        url  = "https://registry.yarnpkg.com/is-extglob/-/is-extglob-2.1.1.tgz";
        sha512 = "SbKbANkN603Vi4jEZv49LeVJMn4yGwsbzZworEoyEiutsN3nJYdbO36zfhGJ6QEDpOZIFkDtnq5JRxmvl3jsoQ==";
      };
    }
    {
      name = "is_fullwidth_code_point___is_fullwidth_code_point_3.0.0.tgz";
      path = fetchurl {
        name = "is_fullwidth_code_point___is_fullwidth_code_point_3.0.0.tgz";
        url  = "https://registry.yarnpkg.com/is-fullwidth-code-point/-/is-fullwidth-code-point-3.0.0.tgz";
        sha512 = "zymm5+u+sCsSWyD9qNaejV3DFvhCKclKdizYaJUuHA83RLjb7nSuGnddCHGv0hk+KY7BMAlsWeK4Ueg6EV6XQg==";
      };
    }
    {
      name = "is_glob___is_glob_4.0.3.tgz";
      path = fetchurl {
        name = "is_glob___is_glob_4.0.3.tgz";
        url  = "https://registry.yarnpkg.com/is-glob/-/is-glob-4.0.3.tgz";
        sha512 = "xelSayHH36ZgE7ZWhli7pW34hNbNl8Ojv5KVmkJD4hBdD3th8Tfk9vYasLM+mXWOZhFkgZfxhLSnrwRr4elSSg==";
      };
    }
    {
      name = "is_number___is_number_7.0.0.tgz";
      path = fetchurl {
        name = "is_number___is_number_7.0.0.tgz";
        url  = "https://registry.yarnpkg.com/is-number/-/is-number-7.0.0.tgz";
        sha512 = "41Cifkg6e8TylSpdtTpeLVMqvSBEVzTttHvERD741+pnZ8ANv0004MRL43QKPDlK9cGvNp6NZWZUBlbGXYxxng==";
      };
    }
    {
      name = "jsonfile___jsonfile_6.1.0.tgz";
      path = fetchurl {
        name = "jsonfile___jsonfile_6.1.0.tgz";
        url  = "https://registry.yarnpkg.com/jsonfile/-/jsonfile-6.1.0.tgz";
        sha512 = "5dgndWOriYSm5cnYaJNhalLNDKOqFwyDB/rr1E9ZsGciGvKPs8R2xYGCacuf3z6K1YKDz182fd+fY3cn3pMqXQ==";
      };
    }
    {
      name = "lilconfig___lilconfig_3.1.3.tgz";
      path = fetchurl {
        name = "lilconfig___lilconfig_3.1.3.tgz";
        url  = "https://registry.yarnpkg.com/lilconfig/-/lilconfig-3.1.3.tgz";
        sha512 = "/vlFKAoH5Cgt3Ie+JLhRbwOsCQePABiU3tJ1egGvyQ+33R/vcwM2Zl2QR/LzjsBeItPt3oSVXapn+m4nQDvpzw==";
      };
    }
    {
      name = "lodash.memoize___lodash.memoize_4.1.2.tgz";
      path = fetchurl {
        name = "lodash.memoize___lodash.memoize_4.1.2.tgz";
        url  = "https://registry.yarnpkg.com/lodash.memoize/-/lodash.memoize-4.1.2.tgz";
        sha512 = "t7j+NzmgnQzTAYXcsHYLgimltOV1MXHtlOWf6GjL9Kj8GK5FInw5JotxvbOs+IvV1/Dzo04/fCGfLVs7aXb4Ag==";
      };
    }
    {
      name = "lodash.uniq___lodash.uniq_4.5.0.tgz";
      path = fetchurl {
        name = "lodash.uniq___lodash.uniq_4.5.0.tgz";
        url  = "https://registry.yarnpkg.com/lodash.uniq/-/lodash.uniq-4.5.0.tgz";
        sha512 = "xfBaXQd9ryd9dlSDvnvI0lvxfLJlYAZzXomUYzLKtUeOQvOP5piqAWuGtrhWeqaXK9hhoM/iyJc5AV+XfsX3HQ==";
      };
    }
    {
      name = "mdn_data___mdn_data_2.0.28.tgz";
      path = fetchurl {
        name = "mdn_data___mdn_data_2.0.28.tgz";
        url  = "https://registry.yarnpkg.com/mdn-data/-/mdn-data-2.0.28.tgz";
        sha512 = "aylIc7Z9y4yzHYAJNuESG3hfhC+0Ibp/MAMiaOZgNv4pmEdFyfZhhhny4MNiAfWdBQ1RQ2mfDWmM1x8SvGyp8g==";
      };
    }
    {
      name = "mdn_data___mdn_data_2.0.30.tgz";
      path = fetchurl {
        name = "mdn_data___mdn_data_2.0.30.tgz";
        url  = "https://registry.yarnpkg.com/mdn-data/-/mdn-data-2.0.30.tgz";
        sha512 = "GaqWWShW4kv/G9IEucWScBx9G1/vsFZZJUO+tD26M8J8z3Kw5RDQjaoZe03YAClgeS/SWPOcb4nkFBTEi5DUEA==";
      };
    }
    {
      name = "micromatch___micromatch_4.0.8.tgz";
      path = fetchurl {
        name = "micromatch___micromatch_4.0.8.tgz";
        url  = "https://registry.yarnpkg.com/micromatch/-/micromatch-4.0.8.tgz";
        sha512 = "PXwfBhYu0hBCPw8Dn0E+WDYb7af3dSLVWKi3HGv84IdF4TyFoC0ysxFd0Goxw7nSv4T/PzEJQxsYsEiFCKo2BA==";
      };
    }
    {
      name = "nanoid___nanoid_3.3.11.tgz";
      path = fetchurl {
        name = "nanoid___nanoid_3.3.11.tgz";
        url  = "https://registry.yarnpkg.com/nanoid/-/nanoid-3.3.11.tgz";
        sha512 = "N8SpfPUnUp1bK+PMYW8qSWdl9U+wwNWI4QKxOYDy9JAro3WMX7p2OeVRF9v+347pnakNevPmiHhNmZ2HbFA76w==";
      };
    }
    {
      name = "node_addon_api___node_addon_api_7.1.1.tgz";
      path = fetchurl {
        name = "node_addon_api___node_addon_api_7.1.1.tgz";
        url  = "https://registry.yarnpkg.com/node-addon-api/-/node-addon-api-7.1.1.tgz";
        sha512 = "5m3bsyrjFWE1xf7nz7YXdN4udnVtXK6/Yfgn5qnahL6bCkf2yKt4k3nuTKAtT4r3IG8JNR2ncsIMdZuAzJjHQQ==";
      };
    }
    {
      name = "node_releases___node_releases_2.0.19.tgz";
      path = fetchurl {
        name = "node_releases___node_releases_2.0.19.tgz";
        url  = "https://registry.yarnpkg.com/node-releases/-/node-releases-2.0.19.tgz";
        sha512 = "xxOWJsBKtzAq7DY0J+DTzuz58K8e7sJbdgwkbMWQe8UYB6ekmsQ45q0M/tJDsGaZmbC+l7n57UV8Hl5tHxO9uw==";
      };
    }
    {
      name = "normalize_path___normalize_path_3.0.0.tgz";
      path = fetchurl {
        name = "normalize_path___normalize_path_3.0.0.tgz";
        url  = "https://registry.yarnpkg.com/normalize-path/-/normalize-path-3.0.0.tgz";
        sha512 = "6eZs5Ls3WtCisHWp9S2GUy8dqkpGi4BVSz3GaqiE6ezub0512ESztXUwUB6C6IKbQkY2Pnb/mD4WYojCRwcwLA==";
      };
    }
    {
      name = "normalize_range___normalize_range_0.1.2.tgz";
      path = fetchurl {
        name = "normalize_range___normalize_range_0.1.2.tgz";
        url  = "https://registry.yarnpkg.com/normalize-range/-/normalize-range-0.1.2.tgz";
        sha512 = "bdok/XvKII3nUpklnV6P2hxtMNrCboOjAcyBuQnWEhO665FwrSNRxU+AqpsyvO6LgGYPspN+lu5CLtw4jPRKNA==";
      };
    }
    {
      name = "nth_check___nth_check_2.1.1.tgz";
      path = fetchurl {
        name = "nth_check___nth_check_2.1.1.tgz";
        url  = "https://registry.yarnpkg.com/nth-check/-/nth-check-2.1.1.tgz";
        sha512 = "lqjrjmaOoAnWfMmBPL+XNnynZh2+swxiX3WUE0s4yEHI6m+AwrK2UZOimIRl3X/4QctVqS8AiZjFqyOGrMXb/w==";
      };
    }
    {
      name = "picocolors___picocolors_1.1.1.tgz";
      path = fetchurl {
        name = "picocolors___picocolors_1.1.1.tgz";
        url  = "https://registry.yarnpkg.com/picocolors/-/picocolors-1.1.1.tgz";
        sha512 = "xceH2snhtb5M9liqDsmEw56le376mTZkEX/jEb/RxNFyegNul7eNslCXP9FDj/Lcu0X8KEyMceP2ntpaHrDEVA==";
      };
    }
    {
      name = "picomatch___picomatch_2.3.1.tgz";
      path = fetchurl {
        name = "picomatch___picomatch_2.3.1.tgz";
        url  = "https://registry.yarnpkg.com/picomatch/-/picomatch-2.3.1.tgz";
        sha512 = "JU3teHTNjmE2VCGFzuY8EXzCDVwEqB2a8fsIvwaStHhAWJEeVd1o1QD80CU6+ZdEXXSLbSsuLwJjkCBWqRQUVA==";
      };
    }
    {
      name = "picomatch___picomatch_4.0.2.tgz";
      path = fetchurl {
        name = "picomatch___picomatch_4.0.2.tgz";
        url  = "https://registry.yarnpkg.com/picomatch/-/picomatch-4.0.2.tgz";
        sha512 = "M7BAV6Rlcy5u+m6oPhAPFgJTzAioX/6B0DxyvDlo9l8+T3nLKbrczg2WLUyzd45L8RqfUMyGPzekbMvX2Ldkwg==";
      };
    }
    {
      name = "pify___pify_2.3.0.tgz";
      path = fetchurl {
        name = "pify___pify_2.3.0.tgz";
        url  = "https://registry.yarnpkg.com/pify/-/pify-2.3.0.tgz";
        sha512 = "udgsAY+fTnvv7kI7aaxbqwWNb0AHiB0qBO89PZKPkoTmGOgdbrHDKD+0B2X4uTfJ/FT1R09r9gTsjUjNJotuog==";
      };
    }
    {
      name = "postcss_calc___postcss_calc_10.1.1.tgz";
      path = fetchurl {
        name = "postcss_calc___postcss_calc_10.1.1.tgz";
        url  = "https://registry.yarnpkg.com/postcss-calc/-/postcss-calc-10.1.1.tgz";
        sha512 = "NYEsLHh8DgG/PRH2+G9BTuUdtf9ViS+vdoQ0YA5OQdGsfN4ztiwtDWNtBl9EKeqNMFnIu8IKZ0cLxEQ5r5KVMw==";
      };
    }
    {
      name = "postcss_cli___postcss_cli_11.0.1.tgz";
      path = fetchurl {
        name = "postcss_cli___postcss_cli_11.0.1.tgz";
        url  = "https://registry.yarnpkg.com/postcss-cli/-/postcss-cli-11.0.1.tgz";
        sha512 = "0UnkNPSayHKRe/tc2YGW6XnSqqOA9eqpiRMgRlV1S6HdGi16vwJBx7lviARzbV1HpQHqLLRH3o8vTcB0cLc+5g==";
      };
    }
    {
      name = "postcss_colormin___postcss_colormin_7.0.2.tgz";
      path = fetchurl {
        name = "postcss_colormin___postcss_colormin_7.0.2.tgz";
        url  = "https://registry.yarnpkg.com/postcss-colormin/-/postcss-colormin-7.0.2.tgz";
        sha512 = "YntRXNngcvEvDbEjTdRWGU606eZvB5prmHG4BF0yLmVpamXbpsRJzevyy6MZVyuecgzI2AWAlvFi8DAeCqwpvA==";
      };
    }
    {
      name = "postcss_convert_values___postcss_convert_values_7.0.4.tgz";
      path = fetchurl {
        name = "postcss_convert_values___postcss_convert_values_7.0.4.tgz";
        url  = "https://registry.yarnpkg.com/postcss-convert-values/-/postcss-convert-values-7.0.4.tgz";
        sha512 = "e2LSXPqEHVW6aoGbjV9RsSSNDO3A0rZLCBxN24zvxF25WknMPpX8Dm9UxxThyEbaytzggRuZxaGXqaOhxQ514Q==";
      };
    }
    {
      name = "postcss_discard_comments___postcss_discard_comments_7.0.3.tgz";
      path = fetchurl {
        name = "postcss_discard_comments___postcss_discard_comments_7.0.3.tgz";
        url  = "https://registry.yarnpkg.com/postcss-discard-comments/-/postcss-discard-comments-7.0.3.tgz";
        sha512 = "q6fjd4WU4afNhWOA2WltHgCbkRhZPgQe7cXF74fuVB/ge4QbM9HEaOIzGSiMvM+g/cOsNAUGdf2JDzqA2F8iLA==";
      };
    }
    {
      name = "postcss_discard_duplicates___postcss_discard_duplicates_7.0.1.tgz";
      path = fetchurl {
        name = "postcss_discard_duplicates___postcss_discard_duplicates_7.0.1.tgz";
        url  = "https://registry.yarnpkg.com/postcss-discard-duplicates/-/postcss-discard-duplicates-7.0.1.tgz";
        sha512 = "oZA+v8Jkpu1ct/xbbrntHRsfLGuzoP+cpt0nJe5ED2FQF8n8bJtn7Bo28jSmBYwqgqnqkuSXJfSUEE7if4nClQ==";
      };
    }
    {
      name = "postcss_discard_empty___postcss_discard_empty_7.0.0.tgz";
      path = fetchurl {
        name = "postcss_discard_empty___postcss_discard_empty_7.0.0.tgz";
        url  = "https://registry.yarnpkg.com/postcss-discard-empty/-/postcss-discard-empty-7.0.0.tgz";
        sha512 = "e+QzoReTZ8IAwhnSdp/++7gBZ/F+nBq9y6PomfwORfP7q9nBpK5AMP64kOt0bA+lShBFbBDcgpJ3X4etHg4lzA==";
      };
    }
    {
      name = "postcss_discard_overridden___postcss_discard_overridden_7.0.0.tgz";
      path = fetchurl {
        name = "postcss_discard_overridden___postcss_discard_overridden_7.0.0.tgz";
        url  = "https://registry.yarnpkg.com/postcss-discard-overridden/-/postcss-discard-overridden-7.0.0.tgz";
        sha512 = "GmNAzx88u3k2+sBTZrJSDauR0ccpE24omTQCVmaTTZFz1du6AasspjaUPMJ2ud4RslZpoFKyf+6MSPETLojc6w==";
      };
    }
    {
      name = "postcss_discard_unused___postcss_discard_unused_7.0.3.tgz";
      path = fetchurl {
        name = "postcss_discard_unused___postcss_discard_unused_7.0.3.tgz";
        url  = "https://registry.yarnpkg.com/postcss-discard-unused/-/postcss-discard-unused-7.0.3.tgz";
        sha512 = "OVxIMVMjkJ1anDTbHFSYUZnmoDWv3vF5JPZvr9hi6HjMNH/RjfR39IMeLThbIjrSb9ZLcwzqziU+XxFQkgF4Vw==";
      };
    }
    {
      name = "postcss_load_config___postcss_load_config_5.1.0.tgz";
      path = fetchurl {
        name = "postcss_load_config___postcss_load_config_5.1.0.tgz";
        url  = "https://registry.yarnpkg.com/postcss-load-config/-/postcss-load-config-5.1.0.tgz";
        sha512 = "G5AJ+IX0aD0dygOE0yFZQ/huFFMSNneyfp0e3/bT05a8OfPC5FUoZRPfGijUdGOJNMewJiwzcHJXFafFzeKFVA==";
      };
    }
    {
      name = "postcss_load_config___postcss_load_config_6.0.1.tgz";
      path = fetchurl {
        name = "postcss_load_config___postcss_load_config_6.0.1.tgz";
        url  = "https://registry.yarnpkg.com/postcss-load-config/-/postcss-load-config-6.0.1.tgz";
        sha512 = "oPtTM4oerL+UXmx+93ytZVN82RrlY/wPUV8IeDxFrzIjXOLF1pN+EmKPLbubvKHT2HC20xXsCAH2Z+CKV6Oz/g==";
      };
    }
    {
      name = "postcss_merge_idents___postcss_merge_idents_7.0.0.tgz";
      path = fetchurl {
        name = "postcss_merge_idents___postcss_merge_idents_7.0.0.tgz";
        url  = "https://registry.yarnpkg.com/postcss-merge-idents/-/postcss-merge-idents-7.0.0.tgz";
        sha512 = "Kr+DniMg0IsW7OGoaMB1Foreb3fIE2XcExCRynogQLngkpNVKTX5GlaxyEZDBB8bISeoztFHFK/GcQtFiPTnpQ==";
      };
    }
    {
      name = "postcss_merge_longhand___postcss_merge_longhand_7.0.4.tgz";
      path = fetchurl {
        name = "postcss_merge_longhand___postcss_merge_longhand_7.0.4.tgz";
        url  = "https://registry.yarnpkg.com/postcss-merge-longhand/-/postcss-merge-longhand-7.0.4.tgz";
        sha512 = "zer1KoZA54Q8RVHKOY5vMke0cCdNxMP3KBfDerjH/BYHh4nCIh+1Yy0t1pAEQF18ac/4z3OFclO+ZVH8azjR4A==";
      };
    }
    {
      name = "postcss_merge_rules___postcss_merge_rules_7.0.4.tgz";
      path = fetchurl {
        name = "postcss_merge_rules___postcss_merge_rules_7.0.4.tgz";
        url  = "https://registry.yarnpkg.com/postcss-merge-rules/-/postcss-merge-rules-7.0.4.tgz";
        sha512 = "ZsaamiMVu7uBYsIdGtKJ64PkcQt6Pcpep/uO90EpLS3dxJi6OXamIobTYcImyXGoW0Wpugh7DSD3XzxZS9JCPg==";
      };
    }
    {
      name = "postcss_minify_font_values___postcss_minify_font_values_7.0.0.tgz";
      path = fetchurl {
        name = "postcss_minify_font_values___postcss_minify_font_values_7.0.0.tgz";
        url  = "https://registry.yarnpkg.com/postcss-minify-font-values/-/postcss-minify-font-values-7.0.0.tgz";
        sha512 = "2ckkZtgT0zG8SMc5aoNwtm5234eUx1GGFJKf2b1bSp8UflqaeFzR50lid4PfqVI9NtGqJ2J4Y7fwvnP/u1cQog==";
      };
    }
    {
      name = "postcss_minify_gradients___postcss_minify_gradients_7.0.0.tgz";
      path = fetchurl {
        name = "postcss_minify_gradients___postcss_minify_gradients_7.0.0.tgz";
        url  = "https://registry.yarnpkg.com/postcss-minify-gradients/-/postcss-minify-gradients-7.0.0.tgz";
        sha512 = "pdUIIdj/C93ryCHew0UgBnL2DtUS3hfFa5XtERrs4x+hmpMYGhbzo6l/Ir5de41O0GaKVpK1ZbDNXSY6GkXvtg==";
      };
    }
    {
      name = "postcss_minify_params___postcss_minify_params_7.0.2.tgz";
      path = fetchurl {
        name = "postcss_minify_params___postcss_minify_params_7.0.2.tgz";
        url  = "https://registry.yarnpkg.com/postcss-minify-params/-/postcss-minify-params-7.0.2.tgz";
        sha512 = "nyqVLu4MFl9df32zTsdcLqCFfE/z2+f8GE1KHPxWOAmegSo6lpV2GNy5XQvrzwbLmiU7d+fYay4cwto1oNdAaQ==";
      };
    }
    {
      name = "postcss_minify_selectors___postcss_minify_selectors_7.0.4.tgz";
      path = fetchurl {
        name = "postcss_minify_selectors___postcss_minify_selectors_7.0.4.tgz";
        url  = "https://registry.yarnpkg.com/postcss-minify-selectors/-/postcss-minify-selectors-7.0.4.tgz";
        sha512 = "JG55VADcNb4xFCf75hXkzc1rNeURhlo7ugf6JjiiKRfMsKlDzN9CXHZDyiG6x/zGchpjQS+UAgb1d4nqXqOpmA==";
      };
    }
    {
      name = "postcss_normalize_charset___postcss_normalize_charset_7.0.0.tgz";
      path = fetchurl {
        name = "postcss_normalize_charset___postcss_normalize_charset_7.0.0.tgz";
        url  = "https://registry.yarnpkg.com/postcss-normalize-charset/-/postcss-normalize-charset-7.0.0.tgz";
        sha512 = "ABisNUXMeZeDNzCQxPxBCkXexvBrUHV+p7/BXOY+ulxkcjUZO0cp8ekGBwvIh2LbCwnWbyMPNJVtBSdyhM2zYQ==";
      };
    }
    {
      name = "postcss_normalize_display_values___postcss_normalize_display_values_7.0.0.tgz";
      path = fetchurl {
        name = "postcss_normalize_display_values___postcss_normalize_display_values_7.0.0.tgz";
        url  = "https://registry.yarnpkg.com/postcss-normalize-display-values/-/postcss-normalize-display-values-7.0.0.tgz";
        sha512 = "lnFZzNPeDf5uGMPYgGOw7v0BfB45+irSRz9gHQStdkkhiM0gTfvWkWB5BMxpn0OqgOQuZG/mRlZyJxp0EImr2Q==";
      };
    }
    {
      name = "postcss_normalize_positions___postcss_normalize_positions_7.0.0.tgz";
      path = fetchurl {
        name = "postcss_normalize_positions___postcss_normalize_positions_7.0.0.tgz";
        url  = "https://registry.yarnpkg.com/postcss-normalize-positions/-/postcss-normalize-positions-7.0.0.tgz";
        sha512 = "I0yt8wX529UKIGs2y/9Ybs2CelSvItfmvg/DBIjTnoUSrPxSV7Z0yZ8ShSVtKNaV/wAY+m7bgtyVQLhB00A1NQ==";
      };
    }
    {
      name = "postcss_normalize_repeat_style___postcss_normalize_repeat_style_7.0.0.tgz";
      path = fetchurl {
        name = "postcss_normalize_repeat_style___postcss_normalize_repeat_style_7.0.0.tgz";
        url  = "https://registry.yarnpkg.com/postcss-normalize-repeat-style/-/postcss-normalize-repeat-style-7.0.0.tgz";
        sha512 = "o3uSGYH+2q30ieM3ppu9GTjSXIzOrRdCUn8UOMGNw7Af61bmurHTWI87hRybrP6xDHvOe5WlAj3XzN6vEO8jLw==";
      };
    }
    {
      name = "postcss_normalize_string___postcss_normalize_string_7.0.0.tgz";
      path = fetchurl {
        name = "postcss_normalize_string___postcss_normalize_string_7.0.0.tgz";
        url  = "https://registry.yarnpkg.com/postcss-normalize-string/-/postcss-normalize-string-7.0.0.tgz";
        sha512 = "w/qzL212DFVOpMy3UGyxrND+Kb0fvCiBBujiaONIihq7VvtC7bswjWgKQU/w4VcRyDD8gpfqUiBQ4DUOwEJ6Qg==";
      };
    }
    {
      name = "postcss_normalize_timing_functions___postcss_normalize_timing_functions_7.0.0.tgz";
      path = fetchurl {
        name = "postcss_normalize_timing_functions___postcss_normalize_timing_functions_7.0.0.tgz";
        url  = "https://registry.yarnpkg.com/postcss-normalize-timing-functions/-/postcss-normalize-timing-functions-7.0.0.tgz";
        sha512 = "tNgw3YV0LYoRwg43N3lTe3AEWZ66W7Dh7lVEpJbHoKOuHc1sLrzMLMFjP8SNULHaykzsonUEDbKedv8C+7ej6g==";
      };
    }
    {
      name = "postcss_normalize_unicode___postcss_normalize_unicode_7.0.2.tgz";
      path = fetchurl {
        name = "postcss_normalize_unicode___postcss_normalize_unicode_7.0.2.tgz";
        url  = "https://registry.yarnpkg.com/postcss-normalize-unicode/-/postcss-normalize-unicode-7.0.2.tgz";
        sha512 = "ztisabK5C/+ZWBdYC+Y9JCkp3M9qBv/XFvDtSw0d/XwfT3UaKeW/YTm/MD/QrPNxuecia46vkfEhewjwcYFjkg==";
      };
    }
    {
      name = "postcss_normalize_url___postcss_normalize_url_7.0.0.tgz";
      path = fetchurl {
        name = "postcss_normalize_url___postcss_normalize_url_7.0.0.tgz";
        url  = "https://registry.yarnpkg.com/postcss-normalize-url/-/postcss-normalize-url-7.0.0.tgz";
        sha512 = "+d7+PpE+jyPX1hDQZYG+NaFD+Nd2ris6r8fPTBAjE8z/U41n/bib3vze8x7rKs5H1uEw5ppe9IojewouHk0klQ==";
      };
    }
    {
      name = "postcss_normalize_whitespace___postcss_normalize_whitespace_7.0.0.tgz";
      path = fetchurl {
        name = "postcss_normalize_whitespace___postcss_normalize_whitespace_7.0.0.tgz";
        url  = "https://registry.yarnpkg.com/postcss-normalize-whitespace/-/postcss-normalize-whitespace-7.0.0.tgz";
        sha512 = "37/toN4wwZErqohedXYqWgvcHUGlT8O/m2jVkAfAe9Bd4MzRqlBmXrJRePH0e9Wgnz2X7KymTgTOaaFizQe3AQ==";
      };
    }
    {
      name = "postcss_ordered_values___postcss_ordered_values_7.0.1.tgz";
      path = fetchurl {
        name = "postcss_ordered_values___postcss_ordered_values_7.0.1.tgz";
        url  = "https://registry.yarnpkg.com/postcss-ordered-values/-/postcss-ordered-values-7.0.1.tgz";
        sha512 = "irWScWRL6nRzYmBOXReIKch75RRhNS86UPUAxXdmW/l0FcAsg0lvAXQCby/1lymxn/o0gVa6Rv/0f03eJOwHxw==";
      };
    }
    {
      name = "postcss_reduce_idents___postcss_reduce_idents_7.0.0.tgz";
      path = fetchurl {
        name = "postcss_reduce_idents___postcss_reduce_idents_7.0.0.tgz";
        url  = "https://registry.yarnpkg.com/postcss-reduce-idents/-/postcss-reduce-idents-7.0.0.tgz";
        sha512 = "ghFHqxigYW/bbfr+bXSDB5Tv3qPaYZZxiQh+Gne0NYRlTOzFft1V/DUvGFVJbFkackHleSjFdVXdlNB+5f3mKg==";
      };
    }
    {
      name = "postcss_reduce_initial___postcss_reduce_initial_7.0.2.tgz";
      path = fetchurl {
        name = "postcss_reduce_initial___postcss_reduce_initial_7.0.2.tgz";
        url  = "https://registry.yarnpkg.com/postcss-reduce-initial/-/postcss-reduce-initial-7.0.2.tgz";
        sha512 = "pOnu9zqQww7dEKf62Nuju6JgsW2V0KRNBHxeKohU+JkHd/GAH5uvoObqFLqkeB2n20mr6yrlWDvo5UBU5GnkfA==";
      };
    }
    {
      name = "postcss_reduce_transforms___postcss_reduce_transforms_7.0.0.tgz";
      path = fetchurl {
        name = "postcss_reduce_transforms___postcss_reduce_transforms_7.0.0.tgz";
        url  = "https://registry.yarnpkg.com/postcss-reduce-transforms/-/postcss-reduce-transforms-7.0.0.tgz";
        sha512 = "pnt1HKKZ07/idH8cpATX/ujMbtOGhUfE+m8gbqwJE05aTaNw8gbo34a2e3if0xc0dlu75sUOiqvwCGY3fzOHew==";
      };
    }
    {
      name = "postcss_reporter___postcss_reporter_7.1.0.tgz";
      path = fetchurl {
        name = "postcss_reporter___postcss_reporter_7.1.0.tgz";
        url  = "https://registry.yarnpkg.com/postcss-reporter/-/postcss-reporter-7.1.0.tgz";
        sha512 = "/eoEylGWyy6/DOiMP5lmFRdmDKThqgn7D6hP2dXKJI/0rJSO1ADFNngZfDzxL0YAxFvws+Rtpuji1YIHj4mySA==";
      };
    }
    {
      name = "postcss_scss___postcss_scss_4.0.9.tgz";
      path = fetchurl {
        name = "postcss_scss___postcss_scss_4.0.9.tgz";
        url  = "https://registry.yarnpkg.com/postcss-scss/-/postcss-scss-4.0.9.tgz";
        sha512 = "AjKOeiwAitL/MXxQW2DliT28EKukvvbEWx3LBmJIRN8KfBGZbRTxNYW0kSqi1COiTZ57nZ9NW06S6ux//N1c9A==";
      };
    }
    {
      name = "postcss_selector_parser___postcss_selector_parser_6.1.2.tgz";
      path = fetchurl {
        name = "postcss_selector_parser___postcss_selector_parser_6.1.2.tgz";
        url  = "https://registry.yarnpkg.com/postcss-selector-parser/-/postcss-selector-parser-6.1.2.tgz";
        sha512 = "Q8qQfPiZ+THO/3ZrOrO0cJJKfpYCagtMUkXbnEfmgUjwXg6z/WBeOyS9APBBPCTSiDV+s4SwQGu8yFsiMRIudg==";
      };
    }
    {
      name = "postcss_selector_parser___postcss_selector_parser_7.1.0.tgz";
      path = fetchurl {
        name = "postcss_selector_parser___postcss_selector_parser_7.1.0.tgz";
        url  = "https://registry.yarnpkg.com/postcss-selector-parser/-/postcss-selector-parser-7.1.0.tgz";
        sha512 = "8sLjZwK0R+JlxlYcTuVnyT2v+htpdrjDOKuMcOVdYjt52Lh8hWRYpxBPoKx/Zg+bcjc3wx6fmQevMmUztS/ccA==";
      };
    }
    {
      name = "postcss_svgo___postcss_svgo_7.0.1.tgz";
      path = fetchurl {
        name = "postcss_svgo___postcss_svgo_7.0.1.tgz";
        url  = "https://registry.yarnpkg.com/postcss-svgo/-/postcss-svgo-7.0.1.tgz";
        sha512 = "0WBUlSL4lhD9rA5k1e5D8EN5wCEyZD6HJk0jIvRxl+FDVOMlJ7DePHYWGGVc5QRqrJ3/06FTXM0bxjmJpmTPSA==";
      };
    }
    {
      name = "postcss_unique_selectors___postcss_unique_selectors_7.0.3.tgz";
      path = fetchurl {
        name = "postcss_unique_selectors___postcss_unique_selectors_7.0.3.tgz";
        url  = "https://registry.yarnpkg.com/postcss-unique-selectors/-/postcss-unique-selectors-7.0.3.tgz";
        sha512 = "J+58u5Ic5T1QjP/LDV9g3Cx4CNOgB5vz+kM6+OxHHhFACdcDeKhBXjQmB7fnIZM12YSTvsL0Opwco83DmacW2g==";
      };
    }
    {
      name = "postcss_value_parser___postcss_value_parser_4.2.0.tgz";
      path = fetchurl {
        name = "postcss_value_parser___postcss_value_parser_4.2.0.tgz";
        url  = "https://registry.yarnpkg.com/postcss-value-parser/-/postcss-value-parser-4.2.0.tgz";
        sha512 = "1NNCs6uurfkVbeXG4S8JFT9t19m45ICnif8zWLd5oPSZ50QnwMfK+H3jv408d4jw/7Bttv5axS5IiHoLaVNHeQ==";
      };
    }
    {
      name = "postcss_zindex___postcss_zindex_7.0.0.tgz";
      path = fetchurl {
        name = "postcss_zindex___postcss_zindex_7.0.0.tgz";
        url  = "https://registry.yarnpkg.com/postcss-zindex/-/postcss-zindex-7.0.0.tgz";
        sha512 = "Agp+5C0qBZxT9S4k9iO/C9oqce3gvPJ/7av4JcAsDl17vsboSN60ncTokIYDtDMlVXvwuhFED3edoy1YG5O1+g==";
      };
    }
    {
      name = "postcss___postcss_8.5.3.tgz";
      path = fetchurl {
        name = "postcss___postcss_8.5.3.tgz";
        url  = "https://registry.yarnpkg.com/postcss/-/postcss-8.5.3.tgz";
        sha512 = "dle9A3yYxlBSrt8Fu+IpjGT8SY8hN0mlaA6GY8t0P5PjIOZemULz/E2Bnm/2dcUOena75OTNkHI76uZBNUUq3A==";
      };
    }
    {
      name = "pretty_hrtime___pretty_hrtime_1.0.3.tgz";
      path = fetchurl {
        name = "pretty_hrtime___pretty_hrtime_1.0.3.tgz";
        url  = "https://registry.yarnpkg.com/pretty-hrtime/-/pretty-hrtime-1.0.3.tgz";
        sha512 = "66hKPCr+72mlfiSjlEB1+45IjXSqvVAIy6mocupoww4tBFE9R9IhwwUGoI4G++Tc9Aq+2rxOt0RFU6gPcrte0A==";
      };
    }
    {
      name = "read_cache___read_cache_1.0.0.tgz";
      path = fetchurl {
        name = "read_cache___read_cache_1.0.0.tgz";
        url  = "https://registry.yarnpkg.com/read-cache/-/read-cache-1.0.0.tgz";
        sha512 = "Owdv/Ft7IjOgm/i0xvNDZ1LrRANRfew4b2prF3OWMQLxLfu3bS8FVhCsrSCMK4lR56Y9ya+AThoTpDCTxCmpRA==";
      };
    }
    {
      name = "readdirp___readdirp_4.1.2.tgz";
      path = fetchurl {
        name = "readdirp___readdirp_4.1.2.tgz";
        url  = "https://registry.yarnpkg.com/readdirp/-/readdirp-4.1.2.tgz";
        sha512 = "GDhwkLfywWL2s6vEjyhri+eXmfH6j1L7JE27WhqLeYzoh/A3DBaYGEj2H/HFZCn/kMfim73FXxEJTw06WtxQwg==";
      };
    }
    {
      name = "readdirp___readdirp_3.6.0.tgz";
      path = fetchurl {
        name = "readdirp___readdirp_3.6.0.tgz";
        url  = "https://registry.yarnpkg.com/readdirp/-/readdirp-3.6.0.tgz";
        sha512 = "hOS089on8RduqdbhvQ5Z37A0ESjsqz6qnRcffsMU3495FuTdqSm+7bhJ29JvIOsBDEEnan5DPu9t3To9VRlMzA==";
      };
    }
    {
      name = "require_directory___require_directory_2.1.1.tgz";
      path = fetchurl {
        name = "require_directory___require_directory_2.1.1.tgz";
        url  = "https://registry.yarnpkg.com/require-directory/-/require-directory-2.1.1.tgz";
        sha512 = "fGxEI7+wsG9xrvdjsrlmL22OMTTiHRwAMroiEeMgq8gzoLC/PQr7RsRDSTLUg/bZAZtF+TVIkHc6/4RIKrui+Q==";
      };
    }
    {
      name = "sass___sass_1.86.3.tgz";
      path = fetchurl {
        name = "sass___sass_1.86.3.tgz";
        url  = "https://registry.yarnpkg.com/sass/-/sass-1.86.3.tgz";
        sha512 = "iGtg8kus4GrsGLRDLRBRHY9dNVA78ZaS7xr01cWnS7PEMQyFtTqBiyCrfpTYTZXRWM94akzckYjh8oADfFNTzw==";
      };
    }
    {
      name = "slash___slash_5.1.0.tgz";
      path = fetchurl {
        name = "slash___slash_5.1.0.tgz";
        url  = "https://registry.yarnpkg.com/slash/-/slash-5.1.0.tgz";
        sha512 = "ZA6oR3T/pEyuqwMgAKT0/hAv8oAXckzbkmR0UkUosQ+Mc4RxGoJkRmwHgHufaenlyAgE1Mxgpdcrf75y6XcnDg==";
      };
    }
    {
      name = "source_map_js___source_map_js_1.2.1.tgz";
      path = fetchurl {
        name = "source_map_js___source_map_js_1.2.1.tgz";
        url  = "https://registry.yarnpkg.com/source-map-js/-/source-map-js-1.2.1.tgz";
        sha512 = "UXWMKhLOwVKb728IUtQPXxfYU+usdybtUrK/8uGE8CQMvrhOpwvzDBwj0QhSL7MQc7vIsISBG8VQ8+IDQxpfQA==";
      };
    }
    {
      name = "source_map___source_map_0.7.4.tgz";
      path = fetchurl {
        name = "source_map___source_map_0.7.4.tgz";
        url  = "https://registry.yarnpkg.com/source-map/-/source-map-0.7.4.tgz";
        sha512 = "l3BikUxvPOcn5E74dZiq5BGsTb5yEwhaTSzccU6t4sDOH8NWJCstKO5QT2CvtFoK6F0saL7p9xHAqHOlCPJygA==";
      };
    }
    {
      name = "string_width___string_width_4.2.3.tgz";
      path = fetchurl {
        name = "string_width___string_width_4.2.3.tgz";
        url  = "https://registry.yarnpkg.com/string-width/-/string-width-4.2.3.tgz";
        sha512 = "wKyQRQpjJ0sIp62ErSZdGsjMJWsap5oRNihHhu6G7JVO/9jIB6UyevL+tXuOqrng8j/cxKTWyWUwvSTriiZz/g==";
      };
    }
    {
      name = "strip_ansi___strip_ansi_6.0.1.tgz";
      path = fetchurl {
        name = "strip_ansi___strip_ansi_6.0.1.tgz";
        url  = "https://registry.yarnpkg.com/strip-ansi/-/strip-ansi-6.0.1.tgz";
        sha512 = "Y38VPSHcqkFrCpFnQ9vuSXmquuv5oXOKpGeT6aGrr3o3Gc9AlVa6JBfUSOCnbxGGZF+/0ooI7KrPuUSztUdU5A==";
      };
    }
    {
      name = "strip_comments___strip_comments_2.0.1.tgz";
      path = fetchurl {
        name = "strip_comments___strip_comments_2.0.1.tgz";
        url  = "https://registry.yarnpkg.com/strip-comments/-/strip-comments-2.0.1.tgz";
        sha512 = "ZprKx+bBLXv067WTCALv8SSz5l2+XhpYCsVtSqlMnkAXMWDq+/ekVbl1ghqP9rUHTzv6sm/DwCOiYutU/yp1fw==";
      };
    }
    {
      name = "stylehacks___stylehacks_7.0.4.tgz";
      path = fetchurl {
        name = "stylehacks___stylehacks_7.0.4.tgz";
        url  = "https://registry.yarnpkg.com/stylehacks/-/stylehacks-7.0.4.tgz";
        sha512 = "i4zfNrGMt9SB4xRK9L83rlsFCgdGANfeDAYacO1pkqcE7cRHPdWHwnKZVz7WY17Veq/FvyYsRAU++Ga+qDFIww==";
      };
    }
    {
      name = "svgo___svgo_3.3.2.tgz";
      path = fetchurl {
        name = "svgo___svgo_3.3.2.tgz";
        url  = "https://registry.yarnpkg.com/svgo/-/svgo-3.3.2.tgz";
        sha512 = "OoohrmuUlBs8B8o6MB2Aevn+pRIH9zDALSR+6hhqVfa6fRwG/Qw9VUMSMW9VNg2CFc/MTIfabtdOVl9ODIJjpw==";
      };
    }
    {
      name = "thenby___thenby_1.3.4.tgz";
      path = fetchurl {
        name = "thenby___thenby_1.3.4.tgz";
        url  = "https://registry.yarnpkg.com/thenby/-/thenby-1.3.4.tgz";
        sha512 = "89Gi5raiWA3QZ4b2ePcEwswC3me9JIg+ToSgtE0JWeCynLnLxNr/f9G+xfo9K+Oj4AFdom8YNJjibIARTJmapQ==";
      };
    }
    {
      name = "tinyglobby___tinyglobby_0.2.12.tgz";
      path = fetchurl {
        name = "tinyglobby___tinyglobby_0.2.12.tgz";
        url  = "https://registry.yarnpkg.com/tinyglobby/-/tinyglobby-0.2.12.tgz";
        sha512 = "qkf4trmKSIiMTs/E63cxH+ojC2unam7rJ0WrauAzpT3ECNTxGRMlaXxVbfxMUC/w0LaYk6jQ4y/nGR9uBO3tww==";
      };
    }
    {
      name = "to_regex_range___to_regex_range_5.0.1.tgz";
      path = fetchurl {
        name = "to_regex_range___to_regex_range_5.0.1.tgz";
        url  = "https://registry.yarnpkg.com/to-regex-range/-/to-regex-range-5.0.1.tgz";
        sha512 = "65P7iz6X5yEr1cwcgvQxbbIw7Uk3gOy5dIdtZ4rDveLqhrdJP+Li/Hx6tyK0NEb+2GCyneCMJiGqrADCSNk8sQ==";
      };
    }
    {
      name = "universalify___universalify_2.0.1.tgz";
      path = fetchurl {
        name = "universalify___universalify_2.0.1.tgz";
        url  = "https://registry.yarnpkg.com/universalify/-/universalify-2.0.1.tgz";
        sha512 = "gptHNQghINnc/vTGIk0SOFGFNXw7JVrlRUtConJRlvaw6DuX0wO5Jeko9sWrMBhh+PsYAZ7oXAiOnf/UKogyiw==";
      };
    }
    {
      name = "update_browserslist_db___update_browserslist_db_1.1.3.tgz";
      path = fetchurl {
        name = "update_browserslist_db___update_browserslist_db_1.1.3.tgz";
        url  = "https://registry.yarnpkg.com/update-browserslist-db/-/update-browserslist-db-1.1.3.tgz";
        sha512 = "UxhIZQ+QInVdunkDAaiazvvT/+fXL5Osr0JZlJulepYu6Jd7qJtDZjlur0emRlT71EN3ScPoE7gvsuIKKNavKw==";
      };
    }
    {
      name = "util_deprecate___util_deprecate_1.0.2.tgz";
      path = fetchurl {
        name = "util_deprecate___util_deprecate_1.0.2.tgz";
        url  = "https://registry.yarnpkg.com/util-deprecate/-/util-deprecate-1.0.2.tgz";
        sha512 = "EPD5q1uXyFxJpCrLnCc1nHnq3gOa6DZBocAIiI2TaSCA7VCJ1UJDMagCzIkXNsUYfD1daK//LTEQ8xiIbrHtcw==";
      };
    }
    {
      name = "wrap_ansi___wrap_ansi_7.0.0.tgz";
      path = fetchurl {
        name = "wrap_ansi___wrap_ansi_7.0.0.tgz";
        url  = "https://registry.yarnpkg.com/wrap-ansi/-/wrap-ansi-7.0.0.tgz";
        sha512 = "YVGIj2kamLSTxw6NsZjoBxfSwsn0ycdesmc4p+Q21c5zPuZ1pl+NfxVdxPtdHvmNVOQ6XSYG4AUtyt/Fi7D16Q==";
      };
    }
    {
      name = "y18n___y18n_5.0.8.tgz";
      path = fetchurl {
        name = "y18n___y18n_5.0.8.tgz";
        url  = "https://registry.yarnpkg.com/y18n/-/y18n-5.0.8.tgz";
        sha512 = "0pfFzegeDWJHJIAmTLRP2DwHjdF5s7jo9tuztdQxAhINCdvS+3nGINqPd00AphqJR/0LhANUS6/+7SCb98YOfA==";
      };
    }
    {
      name = "yaml___yaml_2.7.1.tgz";
      path = fetchurl {
        name = "yaml___yaml_2.7.1.tgz";
        url  = "https://registry.yarnpkg.com/yaml/-/yaml-2.7.1.tgz";
        sha512 = "10ULxpnOCQXxJvBgxsn9ptjq6uviG/htZKk9veJGhlqn3w/DxQ631zFF+nlQXLwmImeS5amR2dl2U8sg6U9jsQ==";
      };
    }
    {
      name = "yargs_parser___yargs_parser_21.1.1.tgz";
      path = fetchurl {
        name = "yargs_parser___yargs_parser_21.1.1.tgz";
        url  = "https://registry.yarnpkg.com/yargs-parser/-/yargs-parser-21.1.1.tgz";
        sha512 = "tVpsJW7DdjecAiFpbIB1e3qxIQsE6NoPc5/eTdrbbIC4h0LVsWhnoa3g+m2HclBIujHzsxZ4VJVA+GUuc2/LBw==";
      };
    }
    {
      name = "yargs___yargs_17.7.2.tgz";
      path = fetchurl {
        name = "yargs___yargs_17.7.2.tgz";
        url  = "https://registry.yarnpkg.com/yargs/-/yargs-17.7.2.tgz";
        sha512 = "7dSzzRQ++CKnNI/krKnYRV7JKKPUXMEh61soaHKg9mrWEhzFWhFnxPxGl+69cD1Ou63C13NUPCnmIcrvqCuM6w==";
      };
    }
  ];
}
