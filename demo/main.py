import os
import pysimplicityhl
import json

def main1():
    basedir = os.path.dirname(__file__)
    hl_file = f"'{basedir}/simple.hl'"
    parameter = [
        "--debug",
        f"{hl_file}",
    ]
    parameter_str = " ".join(parameter)
    result_json= pysimplicityhl.run_from_python(parameter_str)
    res = json.loads(result_json)
    res["parameter_str"] = parameter_str
    print(json.dumps(res, default=str))

def main2():
    basedir = os.path.dirname(__file__)
    hl_file = f"'{basedir}/p2pk.hl'"
    wit_file = f"'{basedir}/p2pk.wit'"
    parameter = [
        "--debug",
        f"{hl_file}",
        f"{wit_file}",
    ]
    parameter_str = " ".join(parameter)
    result_json= pysimplicityhl.run_from_python(parameter_str)
    res = json.loads(result_json)
    res["parameter_str"] = parameter_str
    print(json.dumps(res, default=str))

def main3():
    basedir = os.path.dirname(__file__)
    hl_file = f"'{basedir}/lastwill.hl'"
    wit_file = f"'{basedir}/lastwill.wit'"
    parameter = [
        "--debug",
        f"{hl_file}",
        f"{wit_file}",
    ]
    parameter_str = " ".join(parameter)
    result_json= pysimplicityhl.run_from_python(parameter_str)
    res = json.loads(result_json)
    res["parameter_str"] = parameter_str
    print(json.dumps(res, default=str))


if __name__ == "__main__":
    main1()
    main2()
    main3() 
    
