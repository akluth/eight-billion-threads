%%%-------------------------------------------------------------------
%% @doc eightbillion public API
%% @end
%%%-------------------------------------------------------------------

-module(eightbillion_app).

-behaviour(application).

-export([start/2, stop/1, think/3]).

think(What, 0, 0) ->
    done;
think(What, Mult, Times) ->
    Result = What * Mult,
    think(What, 0, Times - 1).

start(_StartType, _StartArgs) ->
    spawn(eightbillion_app, think, [1337, 10, 8000000000]),
    eightbillion_sup:start_link().

stop(_State) ->
    ok.

%% internal functions
