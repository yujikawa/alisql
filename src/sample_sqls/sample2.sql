select 
u.*
, r.* 
from {{ ref("db", "sales") }} as u
left join {{ ref("db", "sale_detail") }} as r on
u.id = r.sale_id