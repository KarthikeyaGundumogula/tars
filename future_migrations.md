1. **Add profile image column in profiles tables**
   ```sql
    ALTER TABLE profiles ADD COLUMN profile_image_url VARCHAR(255);
   ```
2. **Add text arrays for each image in the scripts table**
   ```sql
    ALTER TABLE scripts ADD COLUMN image_urls TEXT[];
   ```