import os
import subprocess
import sys


def error(error_body):
    print(error_body)
    exit(1)


args = sys.argv
if len(args) == 1:
    error(
        """Usage:
Command filepath [parameters]
        
[parameter] description
        -i  interpret the lang
        -c  compile the lang
        -r  compile and run the lang""")

"""
1 for interpret
2 for compile
3 for compile and run
"""
mode = 0
script_path = args.pop(0)
file = args.pop(0)
if not file.endswith(".sbis"):
    error("Error: Expected a SBIS file")
elif not os.path.exists(file):
    error(f"Error: File '{file}' doesn't exists")
elif os.path.isdir(file):
    error(f"Error: Given Path '{file}' is a directory")

for arg in args:
    match arg:
        case "-i":
            if mode:
                error("Error: expected only 1 mode of running")
            mode = 1

        case "-c":
            if mode:
                error("Error: expected only 1 mode of running")
            mode = 2

        case "-r":
            if mode:
                error("Error: expected only 1 mode of running")
            mode = 3

        case default:
            error(f"Error: Unknown argument {arg}")

match mode:
    case 0:
        error("Error: invalid arguments expected Mode for Running")
    case 1:
        subprocess.run(["./target/release/SB", file, "-i"])
    case 2:
        open(file.replace(".sbis", ".rs"), "x")  # create rust file

        subprocess.run(["./target/release/SB", file, "-c"])
    case 3:
        subprocess.run(["./target/release/SB", file, "-c"])
        subprocess.run(["rustc", file.replace(".sbis", ".rs")])
        subprocess.run([f"./{file.replace('.sbis', '')}"])
    case default:
        error("Error: Unexpected")

