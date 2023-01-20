# Package

version       = "0.1.0"
author        = "Anonymous"
description   = "A new awesome nimble package"
license       = "MIT"
srcDir        = "src"
bin           = @["nimapp"]


# Dependencies

requires "nim >= 1.6.10"

task build, "build nimapp":
  exec("""
    nim c
    -f
    -d:release
    --passC:-flto
    --passL:-flto
    --gc:orc
    src/nimapp
  """)
  
task run, "run nimapp":
  buildTask()
  exec("./src/nimapp")
