1. **Add Episode Upload Functionality**
2. **Add Different Original Tagging For Visual and Music For Works** -- This can be done by the creating a new type on credits table and then a new table for the music since the schema for music is different from the orignals -- keep this on hold for now. - this is a UX challenge we let them credit the music but make sure the ux is not cluttered
3. **Add profile tagging**
4. **Add ability to update the works (Edit platform, number of pages in the script etc.)** - ✅
5. **Right now the permissions are not used sicne we only have two gate keeping actions infuture this may be use full so in that case use the in memory approach to cache at the startup of the application the availbele roles and permissions to optimize for performance**
6. **Allow logging of individual episodes**