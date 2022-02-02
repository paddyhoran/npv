import pytest

from npv import Lives


def test_initial_negative_lives_not_allowed():
    with pytest.raises(ValueError):
        assert Lives(-1)

def test_subtraction_cannot_result_in_negative_lives():
    with pytest.raises(ValueError):
        assert Lives(1) - 2