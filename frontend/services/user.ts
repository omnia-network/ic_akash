import { type User } from "@/declarations/backend.did";
import { extractOk } from "@/helpers/result";
import { BackendActor } from "./backend";

export const getCurrentUser = async (actor: BackendActor): Promise<User> => {
  const res = await actor.get_user();
  return extractOk(res);
};

export const getOrCreateCurrentUser = async (actor: BackendActor): Promise<User> => {
  const res = await actor.get_user();

  let user: User;

  if ("Err" in res) {
    if (res.Err.code === 404) {
      extractOk(await actor.create_user());

      user = await getCurrentUser(actor);
    } else {
      throw res.Err;
    }
  } else {
    user = extractOk(res);
  }

  return user;
};
