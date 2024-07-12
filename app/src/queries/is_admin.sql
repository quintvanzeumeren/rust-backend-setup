select admin
from users
where user_id = $1
limit 1;