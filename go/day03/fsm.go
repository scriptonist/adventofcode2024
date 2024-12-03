package main

import "context"

type State[T any] func(ctx context.Context, args T) (T, State[T], error)

func Run[T any](ctx context.Context, args T, start State[T]) (T, error) {
	var err error
	current := start
	for {
		if ctx.Err() != nil {
			return args, ctx.Err()
		}
		args, current, err = current(ctx, args)
		if err != nil {
			return args, err
		}
		if current == nil {
			return args, nil
		}
	}
}
