# This file is generated by the BAML compiler.
# Do not edit this file directly.
# Instead, edit the BAML files and recompile.
#
# BAML version: 0.1.1-canary.1
# Generated Date: __DATE__
# Generated by: __USER__

# ruff: noqa: E501,F401
# flake8: noqa: E501,F401
# pylint: disable=unused-import,line-too-long
# fmt: off

from ..clients.client_azure_default import AZURE_DEFAULT
from ..functions.fx_messagesimplifier import BAMLMessageSimplifier
from ..types.classes.cls_conversation import Conversation
from ..types.classes.cls_message import Message
from ..types.enums.enm_messagesender import MessageSender
from baml_lib._impl.deserializer import Deserializer
from typing import Optional


# Impl: v1
# Client: AZURE_DEFAULT
# An implementation of .


__prompt_template = """\
Given a chat conversation between a human and ai
simplify the most recent message from the human into a single sentence that includes all prior relevant context. Don't include any previously answered questions. 

{arg}

Most Recent Message:
{arg}

Simplified message:
Human:\
"""

__input_replacers = {
    "{arg}"
}


# We ignore the type here because baml does some type magic to make this work
# for inline SpecialForms like Optional, Union, List.
__deserializer = Deserializer[Optional[int]](Optional[int])  # type: ignore

@BAMLMessageSimplifier.register_impl("v1")
async def v1(arg: Conversation, /) -> Optional[int]:
    response = await AZURE_DEFAULT.run_prompt_template(template=__prompt_template, replacers=__input_replacers, params=dict(arg=arg))
    return __deserializer.from_string(response.generated)
