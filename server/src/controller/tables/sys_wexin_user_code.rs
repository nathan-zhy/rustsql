#[derive(Default)]
#[derive(sqlx::FromRow)]
pub struct SysWexinUserCode
{
    id:i64,
    openid:String,
    code:String,//4
    order_no:String,
    created_by:String,
    phone:String,//11
    created_time:String,
    update_by:String,
    update_time:String
}
