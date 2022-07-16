import subprocess
import sys

"""
python c_r.py filepath.sbis
"""
file = sys.argv[1]

subprocess.run(["cargo", "run", "--", file, "-c"])
subprocess.run(["./" + file.replace(".sbis", "")])