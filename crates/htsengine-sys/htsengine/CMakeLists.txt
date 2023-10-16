cmake_minimum_required(VERSION 3.22)
project(HTSEngineSys)

include(FetchContent)
FetchContent_Declare(
  htsengine
  GIT_REPOSITORY https://github.com/r9y9/hts_engine_API.git
  GIT_TAG 214e26dfb7f728ff9db39c14a59db709abcc121d
  SOURCE_SUBDIR src)
FetchContent_MakeAvailable(htsengine)

# install(TARGETS htsengine LIBRARY DESTINATION lib)
# install(
#   FILES
#     ${htsengine_SOURCE_DIR}/src/jpcommon/jpcommon.h
#     ${htsengine_SOURCE_DIR}/src/mecab/src/mecab.h
#     ${htsengine_SOURCE_DIR}/src/njd/njd.h
#     ${htsengine_SOURCE_DIR}/src/njd2jpcommon/njd2jpcommon.h
#     ${htsengine_SOURCE_DIR}/src/njd_set_accent_phrase/njd_set_accent_phrase.h
#     ${htsengine_SOURCE_DIR}/src/njd_set_accent_type/njd_set_accent_type.h
#     ${htsengine_SOURCE_DIR}/src/njd_set_digit/njd_set_digit.h
#     ${htsengine_SOURCE_DIR}/src/njd_set_long_vowel/njd_set_long_vowel.h
#     ${htsengine_SOURCE_DIR}/src/njd_set_pronunciation/njd_set_pronunciation.h
#     ${htsengine_SOURCE_DIR}/src/njd_set_unvoiced_vowel/njd_set_unvoiced_vowel.h
#     ${htsengine_SOURCE_DIR}/src/text2mecab/text2mecab.h
#     ${htsengine_SOURCE_DIR}/src/mecab2njd/mecab2njd.h
#   DESTINATION include)