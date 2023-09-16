# -*- coding: utf-8 -*-
# ==============================
# 作者: 李健/limoncc
# 邮箱: limoncc@icloud.com
# 日期：2023/9/15 17:13
# 标题：ai run in rust
# 内容：基础知识
# ==============================

import torch


class Test_Model(torch.nn.Module):
    def __init__(self):
        super().__init__()
        self.linear = torch.nn.Linear(in_features=5, out_features=2)
        ...
    def forward(self,inputs:torch.Tensor)->torch.Tensor:
        out = self.linear(inputs)
        outputs = torch.nn.functional.softmax(out,dim=-1)
        return outputs
    ...

test_model = Test_Model()
test_model.eval()
inputs = torch.tensor([1,2,3,4,5],dtype=torch.float32)
with torch.no_grad():
    pt_outputs = test_model(inputs)
print(pt_outputs)

trace_model = torch.jit.trace(test_model, inputs)
trace_model.save("./data/model.jit")


import requests
import json
import time

url = "http://127.0.0.1:3000/users"
data = {"data":[1,2,3,4,5]}

for a in range(10):
    time.sleep(2)
    response = requests.post(url,json=data)
    response.status_code
    x = json.loads(response.text)
    print(x)

