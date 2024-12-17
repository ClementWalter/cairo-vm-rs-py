from vm.cairo_vm_rs_py import sum_as_string


def test_sum_as_string():
    assert sum_as_string(1, 1) == "2"
