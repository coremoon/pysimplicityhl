import os
import pysimplicityhl

def main1():
    basedir = os.path.dirname(__file__)
    hl_file = f"{basedir}/simple.hl"
    parameter = [
        f"{hl_file}",
        "--debug",
    ]
    parameter_str = " ".join(parameter)
    result_json= pysimplicityhl.run_from_python(parameter_str)
    print(result_json)

def main2():
    basedir = os.path.dirname(__file__)
    hl_file = f"{basedir}/lastwill.hl"
    parameter = [
        f"{hl_file}",
        "--debug",
        "--",
        "--param",
        "ALICE_PUBLIC_KEY=0x9bef8d556d80e43ae7e0becb3a7e6838b95defe45896ed6075bb9035d06c9964",
        "BOB_PUBLIC_KEY=0xe37d58a1aae4ba059fd2503712d998470d3a2522f7e2335f544ef384d2199e02",
        "CHARLIE_PUBLIC_KEY=0x688466442a134ee312299bafb37058e385c98dd6005eaaf0f538f533efe5f91f",
    ]
    parameter_str = " ".join(parameter)
    result_json= pysimplicityhl.run_from_python(parameter_str)
    print(result_json)


if __name__ == "__main__":
    main1()
    # main2() # not working yet!!!
    
