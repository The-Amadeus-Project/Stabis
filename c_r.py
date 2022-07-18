import subprocess
import sys

"""
python c_r.py filepath.sbis
"""

if sys.argv == 1:
    print("Expected File Path")
elif len(sys.argv) > 2:
    print("Expected only File Path as argument")
file = sys.argv[1]


return_code = subprocess.run(["cargo", "run", "--", file, "-c"])
if return_code.returncode:
    print(f"ERROR OCCURRED {return_code.stderr}")
else:
    subprocess.run(["./" + file.replace(".sbis", "")])