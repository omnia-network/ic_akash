type GenericResult<T, S> = { 'Ok': T } | { 'Err': S };

export type OkType<T> = T extends { 'Ok': infer U } ? U : never;
export type ErrType<S> = S extends { 'Err': infer U } ? U : never;

export const extractOk = <T, S>(result: GenericResult<T, S>): OkType<GenericResult<T, S>> => {
  if ("Ok" in result) {
    return result.Ok;
  }

  if ("Err" in result) {
    throw result.Err;
  }

  throw new Error("Result does not contain an ok");
};

export const extractErr = <T, S>(result: GenericResult<T, S>): ErrType<GenericResult<T, S>> => {
  if ("Err" in result) {
    return result.Err;
  }

  throw new Error("Result does not contain an error");
};
