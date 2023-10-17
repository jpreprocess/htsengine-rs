cmake_minimum_required(VERSION 3.22)
project(HTSEngineSys)

include(FetchContent)
FetchContent_Declare(
  htsengine
  GIT_REPOSITORY https://github.com/r9y9/hts_engine_API.git
  GIT_TAG 214e26dfb7f728ff9db39c14a59db709abcc121d
  SOURCE_SUBDIR src)
FetchContent_MakeAvailable(htsengine)
