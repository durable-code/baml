# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.
#
# BAML version: 0.0.1
# Generated Date: 2023-10-24 15:23:39.571251 -07:00
# Generated by: vbv

from ..._impl.functions import BaseBAMLFunction
from typing import List, Protocol, Union, runtime_checkable


@runtime_checkable
class IFooBar2(Protocol):
    """
    This is the interface for a function.

    Args:
        arg: Union[List[Int], String]

    Returns:
        List[Union[Int, List[str]]]
    """

    async def __call__(self, arg: Union[List[Int], String], /) -> List[Union[Int, List[str]]]:
        ...


class IBAMLFooBar2(BaseBAMLFunction[List[Union[Int, List[str]]]]):
    def __init__(self) -> None:
        super().__init__(
            "FooBar2",
            IFooBar2,
            ["SomeName"],
        )

BAMLFooBar2 = IBAMLFooBar2()

__all__ = [ "BAMLFooBar2" ]
