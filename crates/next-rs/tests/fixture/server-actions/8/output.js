/* __next_internal_action_entry_do_not_use__ $ACTION_myAction */ async function myAction(
  a,
  b,
  c
) {
  // comment
  "use strict";
  console.log("a");
}
myAction.$$typeof = Symbol.for("react.server.reference");
myAction.$$filepath = "/app/item.js";
myAction.$$name = "$ACTION_myAction";
export const $ACTION_myAction = myAction;
export default function Page() {
  return <Button action={myAction}>Delete</Button>;
}
