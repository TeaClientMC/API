// Import the Elysia library and Database from bun:sqlite
import { Elysia } from 'elysia';
import { Database } from "bun:sqlite";

// Initialize the SQLite database
const db = new Database("TeaClient.sqlite");

// Create the table if it doesn't exist
db.prepare(`
CREATE TABLE IF NOT EXISTS premium_users (
  uuid TEXT PRIMARY KEY,
  starts INTEGER,
  ends INTEGER
)
`);

const app = new Elysia();

app.group('/api', (app) => {
  return app
  .get('/premium', (req, res) => {
    if(req && req.headers && res){
        if (req.headers.uuid) {
            const uuidURL = db.query("SELECT * FROM premium_users WHERE uuid = ?", req.headers.uuid).get();
            try {
                // Query the database
                if (uuidURL) {
                    res.status(200).json({ success: true, uuid: uuidURL.uuid });
                } else {
                    res.status(404).json({ success: false, error: 'User not found' });
                }
            } catch (error) {
                console.error('Error:', error);
                res.status(500).json({ success: false, error: 'Internal server error' });
            }
        } else {
            console.error('Error: Undefined UUID');
            res.status(400).json({ success: false, error: 'UUID is undefined' });
        }
    }
  })
  
.get('/', (req, res) => {
    try {
      const uuid = 'c9e49f5826394be488779f175187f917';
      const starts = Date.now();
      const ends = Date.now() + 1000*60*60*24*30;
      
      // Insert into the database
      db.prepare("INSERT INTO premium_users (uuid, starts, ends) VALUES (?, ?, ?)", [uuid, starts, ends]);
      
      // Query to see if the data is inserted
      const result = db.query("SELECT * FROM premium_users WHERE uuid = ?", uuid).get();
      console.log(result);
      
      Response.json({ success: true, message: 'Data inserted successfully' });
    } catch (error) {
      console.error('Error:', error);
      Response.status(500).json({ success: false, error: 'Internal server error' });
}
});


});

app.listen(process.env.PORT || 3000, () => {
  console.log(`Server is running at ${app.server?.hostname}:${app.server?.port}`);
});



process.on('SIGINT', () => {
  db.close();
  process.exit();
});