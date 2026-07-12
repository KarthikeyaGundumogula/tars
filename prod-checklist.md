1. **change the jwt secret before deployment**
2. **Update the admin logic so that during the startup if the db is empty the pull the admin details from the env and populate them during the startup**
3. **bootstrap some roles and permissions for the app**
4. One more thing worth checking, since you have abort controller wired up
Make sure the abort happens on the client before the request is sent (i.e., cancel the pending timer/request when a new keystroke arrives), not just cancelling the fetch after it's already been dispatched to the server. If you're using debounce + AbortController together — debounce delays firing, AbortController cancels an in-flight one — you get the best of both: fewer requests hit the network at all, and any that do get properly cancelled if superseded.
