#include<bits/stdc++.h>
using namespace std;
 
int main() {
    ios_base::sync_with_stdio(false); cin.tie(NULL);
    string s;
    int cnt,mxcnt =0;
    cnt = mxcnt;
    char cur='Z';
    cin>>s;
    for(int i=0;i<s.length();i++){
        if(cur!=s[i]){
            cur=s[i];
            cnt=1;
        } else{
            cnt++;
        }
        mxcnt=max(cnt,mxcnt);
    }
    cout<<mxcnt;
    return 0;
}